use crate::evaluator::numbers::*;

pub fn interpret(input: String) -> Vec<ExpressionComponents>{
    let mut output_vec: Vec<ExpressionComponents> = vec![];

    let mut chars = input.chars();

    //buffer used to store single number
    let mut buffer = "".to_string();
    let mut peek = chars.clone().next().unwrap();


    //make input iterable by char.
    while let Some(char) = chars.next() {
        println!("char: {}", char);
        if char.is_digit(10) {
            buffer  = char.to_string(); // puts the current char onto the buffer
            peek    = match chars.clone().next() {
                Some(num) => num,
                None => '!', //makes sure while loop wont run if there is no char to peek at
            };

            while peek.is_digit(10) || peek == '.' { //loop through to get full number
                println!("peek: {}", peek);

                //todo negative numbers in interpreter. Currently broken
                buffer = format!("{}{}", buffer, chars.next().unwrap());

                peek = match chars.clone().next() {
                    Some(num) => num,
                    None => {
                        peek = '!';
                        break;
                    },
                };//peek at next value
                println!("buffer: '{}'", buffer);
            }

            match buffer.parse::<f64>() { //push the current number with all digits to the vector
                Ok(num) => {
                    output_vec.push(Type(Float(num)));
                    buffer.clear();
                },

                Err(_) => panic!("Failed to parse buffer: '{}' ", buffer)//TODO error handling as well
            }


            if peek.is_alphabetic() || peek == '(' { // if the next char is a var or (, insert a multiplication in between
                output_vec.push(wrap_buffer('*'));
            }

        } else if char.is_alphabetic() {
            output_vec.push(Type(Variable(Variable { symbol: char, coefficient: 1.0, power: 1.0})));

        } else if char == '/' {
            output_vec.insert(0, Op(LeftParenthesis));  // "Onion" algorithm. adds layers of parenthesis
            output_vec.push(Op(RightParenthesis));      // until the order of operations checks out
            output_vec.push(wrap_buffer(char));

        } else if is_operation(char) {
            output_vec.push(wrap_buffer(char));

        } else if char.is_whitespace() {
            // ignore
        } else {
            panic!("Invalid char, must be a num, variable, or operation");

        }

        peek = match chars.clone().next() {
            Some(num) => num,
            None => break,
        };//peek at next value
    }

    output_vec
}//end of interpret

pub fn is_operation(s: char) -> bool {
    match s {
        '*' | '+' | '/' | '^' |
        '-' | '(' | ')' => true,
        _ => false,
    }
}

pub fn wrap_buffer(s: char) -> ExpressionComponents {
    match s {
        '*' => Op(Multiply) ,
        '+' => Op(Add),
        '/' => Op(Divide), // sorry i missed your initial chat, i ran it a bit earlier and it looks like we just need to fix the interpreter a bit
        '^' => Op(Exponent), // i was thinking we'd be working in input.rs to get it working, was there something wrong in the interpreter?
        '-' => Op(Subtract), // I was running it with just a simple string and having a  problem where when we peek at the next char, it was selecting the same char as the starting char, so if we had "2*3", it would have the current char == 2 and the peek == 2, which meant we would have a problem since we'd put the "*" into the buffer though we didn't want to
        '(' => Op(LeftParenthesis), //that sounds pretty serious, surprised it wasnt caught by the tests
        ')' => Op(RightParenthesis), // what were you planning to do with the input.rs file?
        _   => panic!(), // pretty much just pass input to the interpreter, but maybe do some error checking and stuff
                        // that was the part of the program i was least sure about the design of.
                // let me write a quick test in the interpreter I guess and lets see if its reproducivle
                // sounds good to me, ill get started in input a bit
    }
}// end of wrap_buffer


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multidigit_numbers() {

        let input = String::from("28*32.021"); //lets just test the decimal while we're at it

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![Type(Float(28.0)), Op(Multiply), Type(Float(32.021))];

        assert_eq!(out_vec, cmp_vec);

    }

    #[test]
    fn variables_with_parenthesis() {

        let input = String::from("24x * (32.03 + 12)");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> =
        vec![Type(Float(24.0)), Op(Multiply),
        Type(Variable( Variable { symbol: 'x', coefficient: 1.0, power: 1.0})),
        Op(Multiply),
        Op(LeftParenthesis), Type(Float(32.03)), Op(Add), Type(Float(12.0)), Op(RightParenthesis)];


        assert_eq!(cmp_vec, out_vec);

    }

    #[test]
    fn division_with_variables() {
        let input = String::from("28x / 32.021");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis), Type(Float(28.0)),
        Op(Multiply), Type(Variable(Variable { symbol: 'x', power: 1.0, coefficient: 1.0 })),
        Op(RightParenthesis),
        Op(Divide),
        Type(Float(32.021))];

        assert_eq!(cmp_vec, out_vec);

    }

    #[test]
    fn divide_fractions_with_variables(){
        let input = String::from("3.0 / z^75 / 2.0 / z^2.0");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis),
        Op(LeftParenthesis),
        Op(LeftParenthesis),
        Type(Float(3.0)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Variable(Variable { symbol: 'z', power: 1.0, coefficient: 1.0 })),
        Op(Exponent),
        Type(Float(75.0)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Float(2.0)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Variable(Variable { symbol: 'z', power: 1.0, coefficient: 1.0 })),
        Op(Exponent),
        Type(Float(2.0))];

        assert_eq!(cmp_vec, out_vec);
    }

    #[test]
    fn single_digit_first_num_problem()
    {
        let input = String::from("3*2.3/46");
        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis),
        Op(LeftParenthesis),  ///no shit lol its not working with the tests i never called the fucking function haha
        Type(Float(3.0)),
        Op(Multiply),
        Type(Float(2.3)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Float(46.0)),
        Op(RightParenthesis)
        ];

        let out_vec = interpret(input);
        assert_eq!(cmp_vec, out_vec); // alright im sure this won't pass (almost)
// ah ok now error is reproducing. weird that test is not showing it. try running "2*3"
// yea seems like its an error we "expected", since its our own error message
// not sure what is causing it though

// it is because the peek value is initialized wrong
// if you print it out it is the same as the starting value. It is one value behind the actual valuethat we append to the buffer. So we are appending a "*" to the buffer even though we only want a number
// because there was a number before the *
// not exactly sure on the iterator specifics though.

// interesting, let me change the iterator to print some debug stuff and then run again so i understand.
// its weird to me that it would work with the tests but not doing it manually
    }

}
