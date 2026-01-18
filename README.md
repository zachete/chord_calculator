# Chord Generator

Chord generator web app using a Rust-based WebAssembly module, just to play around.

## Installation

1. Install wasm-pack
```
curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | sh
```

For more details and installation instructions, see the official
[wasm-pack repository](https://github.com/drager/wasm-pack).

2. **Install dependencies**
   ```bash
   npm install
   ```

2. **Build the WASM module**
   ```bash
   npm run build:wasm
   ```

3. **Start the development server**
   ```bash
   npm run dev
   ```
