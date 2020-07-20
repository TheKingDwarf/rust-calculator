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
        // how can we print this in a non-ugly way? we need like the interpreter in reverse lol
        // nah we just need a big ugly match statement i think
        // you right, maybe we should put it in input.rs so there's a bit more stuff in there // yeah idk feel kind of like i would clown on somebody for putting the output
        // in the input file but also is it worth making an output.rs lol?

        // definitely kinda clowny to put output in the input file lol
            // maybe a good idea to change the name of input.rs to io.rs? hjah hell yeah clown free 🤡

        // gott make sure we add the clown emoji to the match statement 🤡🤡
        // yeah lets do it lol

        // lets just use this to mark code that sucks 🤡

        // i feel like these teletype conversations would make great memes for anyone that had to maintain this codebase lmao
        // absolutely, maybe we should have been leaving these in for posterity haha

        // we can leave this one in this commit and tthen get rid of it later lol
        // sounds good to me, hi Thaddeus if you see this!
        // hi Thaddeus!

    } //repeat
}

// would you like to push?
// should compile now
// oh okay, good idea
