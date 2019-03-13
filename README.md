# Rust WASM template

This is my basic opinionated template for bootstrapping Rust Wasm applications.

## Setup

`npm install`

## Usage

Serve:
`npm run serve`

Build:
`npm run build`

Check:
`npm run clippy`

## Arrangement Of The Template

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

