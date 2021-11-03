# markdown-editor
Markdown editor built using Rust WebAssembly.

## How to use
- Run `python3 -m http.server`
  > Note: Use any web server supporting wasm.
- Visit [localhost](http://localhost:8000/public).

## Development
- Run `wasm-pack build --target web` to compile rust into wasm.
