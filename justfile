# -*-Makefile-*-

test colours='':
     cargo {{colours}} nextest run

test-verbose colours='':
     cargo {{colours}} nextest run --no-capture

build:
    cargo build

clean:
    cargo clean

debug *args:
    cargo run           --bin sentinel -- {{args}}

release *args:
    cargo run --release --bin sentinel -- {{args}}
