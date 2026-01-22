use std::collections::HashMap;

const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub fn calc_chord(root: &str, chord_type: &str) -> Result<Vec<String>, String> {
    let chord_indexes = calc_chord_indexes(root, chord_type)?;
    let chord_notes = chord_indexes
        .into_iter()
        .map(|idx| NOTES[idx].to_string())
        .collect();

    Ok(chord_notes)
}

pub fn calc_chord_indexes(root: &str, chord_type: &str) -> Result<Vec<usize>, String> {
    let chords: HashMap<&'static str, Vec<i8>> = HashMap::from([
        ("maj", vec![0, 4, 7]),
        ("min", vec![0, 3, 7]),
        ("dim", vec![0, 3, 6]),
        ("maj7", vec![0, 4, 7, 11]),
        ("min7", vec![0, 3, 7, 10]),
        ("dom7", vec![0, 4, 7, 10]),
    ]);

    let formula = chords
        .get(chord_type)
        .ok_or_else(|| format!("Unknown chord type: {chord_type}"))?;

    let root_index = note_to_index(root).ok_or_else(|| format!("Unknown note: {root}"))?;
    let mut chord_indexes = Vec::with_capacity(formula.len());

    for &step in formula {
        let idx = ((root_index as i8 + step).rem_euclid(12)) as usize;
        chord_indexes.push(idx);
    }

    Ok(chord_indexes)
}

pub fn calc_string_frets(string_note_index: usize, chord_notes_indexes: &[usize]) -> Vec<usize> {
    const FRETS: usize = 12;
    let mut fret_positions = Vec::with_capacity(chord_notes_indexes.len());

    for &chord_note_index in chord_notes_indexes {
        let fret = if chord_note_index >= string_note_index {
            chord_note_index - string_note_index
        } else {
            chord_note_index + FRETS - string_note_index
        };

        fret_positions.push(fret);
    }

    fret_positions
}

pub fn calc_guitar_neck(root: &str, chord_type: &str) -> Result<Vec<Vec<usize>>, String> {
    let tuning = ["E", "A", "D", "G", "B", "E"];
    let mut guitar_matrix = Vec::with_capacity(tuning.len());

    let chord_indexes = calc_chord_indexes(root, chord_type)?;

    for string_note in tuning {
        let string_note_index =
            note_to_index(string_note).ok_or_else(|| format!("Unknown note: {string_note}"))?;

        let string_frets = calc_string_frets(string_note_index, &chord_indexes);
        guitar_matrix.push(string_frets);
    }

    Ok(guitar_matrix)
}

fn note_to_index(note: &str) -> Option<usize> {
    match note {
        "C" => Some(0),
        "C#" => Some(1),
        "Db" => Some(1),
        "D" => Some(2),
        "D#" => Some(3),
        "Eb" => Some(3),
        "E" => Some(4),
        "F" => Some(5),
        "F#" => Some(6),
        "Gb" => Some(6),
        "G" => Some(7),
        "G#" => Some(8),
        "Ab" => Some(8),
        "A" => Some(9),
        "A#" => Some(10),
        "Bb" => Some(10),
        "B" => Some(11),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_chords() {
        let result = calc_chord("C", "maj").unwrap();
        assert_eq!(result, vec!["C", "E", "G"]);

        let result = calc_chord("G", "min").unwrap();
        assert_eq!(result, vec!["G", "A#", "D"]);
    }

    #[test]
    fn test_chord_indexes() {
        let result = calc_chord_indexes("C", "maj").unwrap();
        assert_eq!(result, vec![0, 4, 7]);
    }

    #[test]
    fn test_single_string_frets() {
        let e_string_index = 4; // E note
        let c_major_notes = vec![0, 4, 7]; // C, E, G

        let result = calc_string_frets(e_string_index, &c_major_notes);

        // Expected: C=8th fret, E=0th fret, G=3rd fret
        assert_eq!(result, vec![8, 0, 3]);

        println!("E string frets for C major: {:?}", result);
    }

    #[test]
    fn test_simple_calculation() {
        let g_string_index = 7;
        let g_note = vec![7];
        let result = calc_string_frets(g_string_index, &g_note);

        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_guitar_matrix() {
        let matrix = calc_guitar_neck("C", "maj").unwrap();

        assert_eq!(matrix.len(), 6);
        assert!(matrix.iter().all(|string| string.len() == 3));

        let expected = [
            [8, 0, 3],  // E string: C=8th, E=0th, G=3rd fret
            [3, 7, 10], // A string
            [10, 2, 5], // D string
            [5, 9, 0],  // G string
            [1, 5, 8],  // B string
            [8, 0, 3],  // E string (same as first)
        ];

        for (i, (actual, expected)) in matrix.iter().zip(expected.iter()).enumerate() {
            assert_eq!(actual, expected, "String {} mismatch", i + 1);
        }
    }
}
