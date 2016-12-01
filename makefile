
.PHONY: clean

all: build/brainfuck

build/brainfuck: brainfuck.rs
	mkdir -p build
	rustc brainfuck.rs -o build/brainfuck

clean: 
	rm -rf build
