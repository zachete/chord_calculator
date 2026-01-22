import { Fretboard } from "@moonwave99/fretboard.js";
import { html, render } from "uhtml";
import { mapFretboard } from "./utils.js";
import init, { calc_chord, calc_guitar_neck } from "./wasm/wasm.js";

await init();

let fretboard: Fretboard;

const form = document.getElementById("chord-form") as HTMLFormElement;
const rootSelect = document.getElementById("root") as HTMLSelectElement;
const chordTypeSelect = document.getElementById(
	"chord-type",
) as HTMLSelectElement;
const resultDiv = document.getElementById("result") as HTMLDivElement;

form.addEventListener("submit", (event) => {
	event.preventDefault();

	const root = rootSelect.value;
	const chordType = chordTypeSelect.value;

	if (!fretboard) {
		fretboard = new Fretboard({
			instrument: "guitar",
			markerStyle: "dots",
			el: "#fretboard",
			noteNameStyle: "sharp",
			fretCount: 12,
			height: 200,
		});
	}

	try {
		const { chord, guitarNeck } = getChordData(root, chordType);

		render(resultDiv, renderResult(root, chordType, chord));

		const fretboardMap = mapFretboard(guitarNeck);

		fretboard.setDots(fretboardMap).render().style({
			fill: "red",
		});
	} catch {
		render(resultDiv, renderError("Could not calculate chord."));
	}

	resultDiv.classList.remove("hidden");
});

function getChordData(root: string, chordType: string) {
	const chord = calc_chord(root, chordType);
	const guitarNeckRaw = calc_guitar_neck(root, chordType);
	const guitarNeck: number[][] = JSON.parse(guitarNeckRaw);

	return { chord, guitarNeck };
}

function renderResult(root: string, chordType: string, chord: string[]) {
	return html`
    <h3 class="text-lg font-semibold text-gray-800 mb-2">${root} ${chordType}:</h3>
    <p class="text-2xl font-mono text-blue-600">${chord.join(", ")}</p>
  ` as HTMLElement;
}

function renderError(error: string) {
	return html`<p class="text-red-600 font-medium">${error}</p>` as HTMLElement;
}
