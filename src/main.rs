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
        // println!("{:?}", in_vec); // would you like to check this quickly? yes

        // evaluate
        let out_vec = crate::evaluator::evaluate_stack(&mut in_vec);
        // println!("{:?}", out_vec);

        // display
        println!("{}", crate::io::format_expression(out_vec));

    } //repeat
}
