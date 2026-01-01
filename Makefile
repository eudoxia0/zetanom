BIN := zetanom
SRC := Cargo.toml Cargo.lock $(shell find src -name '*.rs')

all: $(BIN)

$(BIN): $(SRC)
	cargo build --release
	cp target/release/$(BIN) $(BIN)

install: $(BIN)
	systemctl --user stop $(BIN)
	cp $(BIN) ~/.eudoxia.d/bin/$(BIN)
	systemctl --user start $(BIN)

watch:
	cargo watch -x "run -- serve"

clean:
	cargo clean
	rm -f $(BIN)
