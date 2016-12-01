SRC = brainfuck.rs



compile: ${SRC} 
	rustc brainfuck.rs -o build/brainfuck
