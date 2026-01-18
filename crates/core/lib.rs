use std::collections::HashMap;

pub fn calc_chord(root: &str, chord_type: &str) -> Result<Vec<String>, String> {
    const NOTES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let chords: HashMap<&'static str, Vec<i32>> = HashMap::from([
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
    let mut out = Vec::with_capacity(formula.len());

    for &step in formula {
        let idx = ((root_index as i32 + step).rem_euclid(12)) as usize;
        out.push(NOTES[idx].to_string());
    }

    Ok(out)
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
