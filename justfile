# -*-Makefile-*-

test:
     cargo nextest run --profile fast

verbose regexp:
     cargo nextest run --no-capture --profile full -E "test({{regexp}})"

test-slow:
     cargo nextest run --release --profile full

build:
    cargo build

clean:
    cargo clean

debug *args:
    cargo run           --bin sentinel -- {{args}}

release *args:
    cargo run --release --bin sentinel -- {{args}}
