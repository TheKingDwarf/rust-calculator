An interpretative Calculator project written in Rust.

Program Structure (abstract):

Recieve Input --> Interpret Input to Workable instructions -->

 perform mathmetical Operations --> Print Output


Module Structure/ Architecture:

src/
	main.rs -- runs input loop, feeds input through interpreter + calculations, outputs.
	
	input.rs -- error handling on input

	interpreter.rs (low priority) -- Translates the user input to mathemetical instructions

	evaluator/ -- Takes the input code, and performs the required Operations
		mod.rs -- "main" file, which loops through the input and chooses which Operations to perform. Most importantly determines order of Operations
		
		Operations/
			mod.rs -- imports all Operations modules into one big Operations file
		
			arithmetic.rs

			trig.rs
	
			etc..
