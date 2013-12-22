RUSTC ?= rustc
RUSTFLAGS ?=
RUST-SDL2-DIR ?= rust-sdl2
RUST-SDL2-LIBDIR ?= $(RUST-SDL2-DIR)/build/*/sdl2

platformers: build
	./platformers

deps:
	make -C $(RUST-SDL2-DIR)

build: src/platformer/platformer.rs deps
	$(RUSTC) -L . -L $(RUST-SDL2-LIBDIR) $< -o platformers

.PHONY: clean
clean:
	rm -f platformers

