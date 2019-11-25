An interpretative Calculator project written in Rust.

Program Structure (abstract):

Recieve Input --> Interpret Input to Workable instructions -->

 perform mathmetical operations --> Print Output


Module Structure/ Architecture:

src/
	main.rs -- runs input loop, feeds input through interpreter + calculations, outputs.
	
	input.rs -- error handling on input

	interpreter.rs (low priority) -- Translates the user input to mathemetical instructions

	evaluator/ -- Takes the input code, and performs the required operations
		mod.rs -- "main" file, which loops through the input and chooses which operations to perform. Most importantly determines order of operations
		
		operations/
			mod.rs -- imports all operations modules into one big operations file
		
			arithmetic.rs

			trig.rs
	
			etc..
