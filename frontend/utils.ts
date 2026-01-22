export function mapFretboard(rawFret: number[][]) {
	const result = [];
	const stringsCount = rawFret.length;

	for (const [stringIndex, stringFrets] of rawFret.reverse().entries()) {
		for (const fret of stringFrets) {
			result.push({
				string: stringsCount - stringIndex,
				fret,
			});
		}
	}

	return result;
}
