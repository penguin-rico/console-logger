PREFIX ?= /usr/local
BIN = console-logger

all: ## Compile
	cargo build --release

install: ## Compile and Install
	install -Dm755 target/release/$(BIN) $(DESTDIR)$(PREFIX)/bin/$(BIN)

uninstall: ## Removes binary
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BIN)

purge: uninstall ## Removes .config/console-logger & ~/console-logger
	rm -rf $(HOME)/.config/console-logger
	rm -rf $(HOME)/console-logger

help: ## Show this help
	@grep -E '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | \
		awk -F':.*## ' '{printf "  %-10s %s\n", $$1, $$2}'

clean: # Cargo clean
	cargo clean

.PHONY: all install uninstall clean
