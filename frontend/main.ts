import init, { calc_chord } from "./wasm/wasm.js";

await init();

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

  try {
    const chord = calc_chord(root, chordType);

    resultDiv.innerHTML = `
      <h3 class="text-lg font-semibold text-gray-800 mb-2">${root} ${chordType}:</h3>
      <p class="text-2xl font-mono text-blue-600">${chord.join(", ")}</p>
    `;

    resultDiv.classList.remove("hidden");
  } catch (error) {
    resultDiv.innerHTML = `<p class="text-red-600 font-medium">Error: ${error}</p>`;
    resultDiv.classList.remove("hidden");
  }
});

const initialChord = calc_chord("A", "maj");

console.log(initialChord);
