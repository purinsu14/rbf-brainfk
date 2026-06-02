TARGET = rbf
BINARY = brainfkinterpreter
RELEASE_DIR = target/release

.PHONY: build install clean run uninstall

build:
	cargo build --release
	cp $(RELEASE_DIR)/$(BINARY) $(RELEASE_DIR)/$(TARGET)

install: build
	mkdir -p ~/.local/bin
	cp $(RELEASE_DIR)/$(TARGET) ~/.local/bin/

clean:
	cargo clean

run: build
	./$(RELEASE_DIR)/$(TARGET) $(ARGS)

uninstall:
	rm -rf ~/.local/bin/$(TARGET)
