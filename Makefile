.PHONY: all build release install test check clean

all: release

build:
	cargo build

release:
	cargo build --release

install: release
	install -d $(HOME)/.local/bin
	install -m 755 target/release/tempus $(HOME)/.local/bin/tempus
	install -d $(HOME)/.local/share/applications
	install -m 644 assets/tempus.desktop $(HOME)/.local/share/applications/tempus.desktop
	@update-desktop-database $(HOME)/.local/share/applications 2>/dev/null || true
	@echo "✨ Tempus instalado com sucesso em ~/.local/bin/tempus"

test:
	cargo test

check:
	cargo clippy -- -D warnings

clean:
	cargo clean
