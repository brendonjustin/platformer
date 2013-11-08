RUSTC ?= rustc
RUSTFLAGS ?=
RUST-SDL2-DIR ?= rust-sdl2

platformers: build
	./platformers

deps:
	make -C $(RUST-SDL2-DIR)

build: src/platformer/platformer.rc deps
	$(RUSTC) -L . -L $(RUST-SDL2-DIR) $< -o platformers

.PHONY: clean
clean:
	rm -f platformers

