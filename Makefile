RUSTC ?= rustc
RUSTFLAGS ?=
RUST-SDL2-DIR ?= rust-sdl2

platformers: src/platformer/platformer.rc deps
	$(RUSTC) -L . -L $(RUST-SDL2-DIR) $< -o $@
	./platformers

deps:
	make -C $(RUST-SDL2-DIR)

.PHONY: clean
clean:
	rm -f platformers

