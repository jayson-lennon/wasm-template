# Rust WASM template

This is my basic template for bootstrapping Rust Wasm applications.

## Setup

```
git clone https://github.com/jayson-lennon/wasm-template
cd wasm-template
npm install
```

## Usage

Serve:
`npm run serve`

Build:
`npm run build`

Check:
`npm run clippy`

## Template Layout

```
/
| src
  | crate
    | ... Rust code
  | web
    | index.pug
    | js
      | ... Wasm loader & any custom JS
    | style
      | ... Sass files
```

