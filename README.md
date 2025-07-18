# ClayDash

A simple dashboard using the [clay](https://github.com/nicbarker/clay) layout library. This project's purpose was to familiarize myself with Rust's FFI capabilities.


The dashboard shows a histogram of simulations of probability distribution functions and/or their respective probability density function.


The UI is built using the [clay](https://github.com/nicbarker/clay) and [raylib](https://github.com/raysan5/raylib) libraries and the code leverages [one of the examples](https://github.com/nicbarker/clay/tree/main/examples/raylib-multi-context) in clay's GitHub page. This very far from a full set of bindings to clay's C code, I just used what I needed.


https://github.com/user-attachments/assets/e7e580cc-2aa6-4c0e-9e50-575a69203e04


## Running

You need to have a compiled version of raylib to run this project. For this project I used raylib version 5.5, if you are going to use a different version you may need to download a new set of header files that match your library version.

Where you save your library must match what the file `build.rs` has, so you either will have to amend the `build.rs` or save it in the location the file already looks for it.

By default the `build.rs` tries to link with raylib statically.


## Hot Reloading

If you want to enable hot reloading (i.e. make changes to the application while it is running), you need to run/compile the project with the feature flag __hot_reload__ (e.g. `cargo run --features hot_reload`).

At the moment this only works on Linux as I have not looked into how to load a DLL dynamically on Windows.
