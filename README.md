[![Build Status](https://github.com/clay-ui-rs/clay/actions/workflows/ci.yaml/badge.svg)](https://github.com/clay-ui-rs/clay/actions?workflow=Rust%20CI)
[![Crates.io](https://img.shields.io/crates/v/clay-layout.svg)](https://crates.io/crates/clay-layout)
[![Documentation](https://docs.rs/clay-layout/badge.svg)](https://docs.rs/clay-layout)

# Clay Rust Bindings

Rust bindings for [Clay](https://github.com/nicbarker/clay), a UI layout library written in C.

Usage
-----

```toml
# Cargo.toml
[dependencies]
clay-layout = "0.4"
```

Example usage:

```rust
use clay_layout::{fixed, Clay, Declaration};

fn main() {
    // Create the clay instance
    let mut clay = Clay::new((800., 600.).into());

    // Begin the layout
    let mut clay = clay.begin::<(), ()>();

    // Adds a red rectangle with a corner radius of 5.
    // The Layout makes the rectangle have a width and height of 50.
    clay.with(&Declaration::new()
        .id(clay.id("red_rectangle"))
        .layout()
            .width(fixed!(50.))
            .height(fixed!(50.))
        .end()
        .corner_radius()
            .all(5.)
            .end()
        .background_color((0xFF, 0x00, 0x00).into()), |_| {},
    );

    // Return the list of render commands of your layout
    let render_commands = clay.end();

    for command in render_commands {
        println!("Id of the element: {}", command.id); // Note: Ids are in fact numbers generated by Clay
        println!("Bounding box: {:?}", command.bounding_box);
        println!("Type and config: {:?}", command.config);
    }
}
```

## Examples

Examples can be found in the `examples/` directory. They can be ran using `cargo`:

```sh
cargo run --example basic_rectangle
cargo run --example raylib_renderer --features raylib-renderer
```

## Build bindings

Notice that building the bindings is only required if you update the `clay.h` \
To build bindings you need to use the `generate_bindings` script. \
It needs `bindgen` installed as a CLI, you can install it with `cargo install bindgen`. \
Calling it will use the `clay.h` in the project root, or any `clay.h` file provided with `CLAY_HEADER_PATH`. \
Using the clay header it will generate `src/bindings/bindings.rs` and `src/bindings/bindings_debug.rs`.
