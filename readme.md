# Virtual Clock
## How to compile
firstly you need [Rust](https://www.rust-lang.org/tools/install)

Then you CD into this directory and do `cargo run --release` to compile and run the program (it will take awhile the first time)

If you're on linux, you might need to install something extra, see [this link and scroll down to where it says linux](https://crates.io/crates/macroquad)

## Controls
* Up/Down arrow: adjust simulation speed in normal mode
* F: toggle turbo mode
* R: toggle reverse in turbo mode
* Left/Right arrow: adjust blend between max hour hand speed (100% blend) and correct speed relative to minute hand (0% blend). Turbo mode only.