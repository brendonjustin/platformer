RUSTC ?= rustc
RUSTFLAGS ?=

platformers: src/platformer/platformer.rc
	$(RUSTC) -L . $< -o $@
	./platformers

.PHONY: clean
clean:
	rm -f platformers

