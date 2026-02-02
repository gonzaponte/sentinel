# -*-Makefile-*-

test colours='':
     cargo {{colours}} nextest run

build:
    cargo build

clean:
    cargo clean

debug *args:
    cargo run           --bin sentinel -- {{args}}

release *args:
    cargo run --release --bin sentinel -- {{args}}
