mod evaluator;
mod interpreter;
mod io;

fn main() {
    loop {
        // get an input command 🤡
        println!("Enter: ");
        let input = crate::io::get_line();

	// interpret the command
        let mut in_vec = crate::interpreter::interpret(input);

        // evaluate
        let out_vec = crate::evaluator::evaluate_stack(&mut in_vec);

        // display
        println!("{}", crate::io::format_expression(out_vec));

    } //repeat
}
