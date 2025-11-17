watch:
    cargo watch -x "run -- serve"

install:
    cargo build --release
    cp target/release/zetanom ~/.eudoxia.d/bin/zetanom
