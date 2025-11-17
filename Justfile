watch:
    cargo watch -x "run -- serve"

install:
    cargo build --release
    systemctl --user stop zetanom
    cp target/release/zetanom ~/.eudoxia.d/bin/zetanom
    systemctl --user start zetanom
