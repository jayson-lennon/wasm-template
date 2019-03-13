#![allow(clippy::pedantic)]

#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::web::document;

fn main() {
    let hello = document().create_element("p").unwrap();
    hello.set_text_content("Hello from Wasm");
    document().body().unwrap().append_child(&hello);

    js! { @(no_return)
        console.log("hello console");
    }
}
