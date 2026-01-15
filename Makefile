# Makefile for acty

CARGO = cargo
BIN_NAME = acty
INSTALL_DIR = $(HOME)/.local/bin
TARGET_BIN = target/release/$(BIN_NAME)

.PHONY: all
all: build

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: install
install: build
	mkdir -p $(INSTALL_DIR)
	cp $(TARGET_BIN) $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Installed $(BIN_NAME) to $(INSTALL_DIR)"

.PHONY: test
test:
	$(CARGO) test

.PHONY: check
check:
	$(CARGO) check

.PHONY: clean
clean:
	$(CARGO) clean
