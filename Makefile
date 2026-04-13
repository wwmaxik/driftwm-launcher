.PHONY: build install uninstall clean run

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

build:
	@command -v cargo >/dev/null 2>&1 || { echo "cargo not found in PATH"; exit 1; }
	cargo build --release

install:
	@test -f target/release/driftwm-launcher || { echo "Binary not found. Run 'cargo build --release' first."; exit 1; }
	install -Dm755 target/release/driftwm-launcher $(DESTDIR)$(BINDIR)/driftwm-launcher

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/driftwm-launcher

clean:
	@command -v cargo >/dev/null 2>&1 && cargo clean || rm -rf target

run:
	@command -v cargo >/dev/null 2>&1 || { echo "cargo not found in PATH"; exit 1; }
	cargo run
