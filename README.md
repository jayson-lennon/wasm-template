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
|-- src
  |-- crate
      |--  # Rust code lives here
  |-- web
      |-- index.pug     # web app entry point
      |-- js
          |-- # Wasm loader & any custom JS goes here
      |-- style
          |-- # Sass files here
```

