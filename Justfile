default: run-dyn

alias r := run
run:
    cargo run

alias rr := run-release
# Run with the `--release` flag
run-release:
    cargo run --release

alias rd := run-dyn
# Run with dynamic linking
run-dyn:
    cargo run --features bevy/dynamic_linking

pack:
    upx --best --lzma target/x86_64-unknown-linux-gnu/release/bevy-breakout-tut -o target/x86_64-unknown-linux-gnu/release/bevy-breakout-tut-optimized

alias rp := run-packed
run-packed:
    ./target/x86_64-unknown-linux-gnu/release/bevy-breakout-tut-optimized
