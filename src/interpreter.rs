use crate::evaluator::numbers::*;

pub fn interpret(mut input: String) -> Vec<ExpressionComponents>{
    let mut output_vec: Vec<ExpressionComponents> = vec![];

    input.retain(|c| !c.is_whitespace());
    let mut chars = input.chars();

    //buffer used to store single number
    let mut buffer = "".to_string();

    // if chars is empty then the output vec will be empty,
    // and this line will not error since we won't unwrap a "None" value
    let mut peek: char = ' ';
    if !chars.as_str().is_empty() {
	peek = chars.clone().next().unwrap();
    }

    if (peek == '-') {
        buffer += &chars.next().unwrap().to_string();
    }

    let mut stored_coefficient = 1.0;

    //make input iterable by char.
    while let Some(char) = chars.next() {
        println!("char: {}", char);
        if char.is_digit(10) {
            buffer  += &char.to_string(); // puts the current char onto the buffer
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
                    if peek.is_alphabetic() {
                        stored_coefficient = num;
                    } else {
                        output_vec.push(Type(Float(num)));
                    }
                },

                Err(_) => panic!("Failed to parse buffer: '{}' ", buffer)//TODO error handling as well
            }


            if peek == '(' { // if the next char is a var or (, insert a multiplication in between
                output_vec.push(wrap_buffer('*'));
            }

            buffer.clear();

        } else if char.is_alphabetic() {
            match chars.clone().next() {
                Some('^') => {
                    let mut peek_chars = chars.clone();
                    peek_chars.next().unwrap();

                    if peek_chars.clone().next().unwrap() == '-' {
                        buffer += &peek_chars.next().unwrap().to_string();
                    }

                    while let Some(char_inner) = peek_chars.next() {
                        if char_inner.is_digit(10) || char_inner == '.' {
                            buffer += &char_inner.to_string();
                        }
                        else {
                            break;
                        }
                    }

                    match buffer.parse::<f64>() { //push the current number with all digits to the vector
                        Ok(num) => {
                            output_vec.push(Type(Variable(Variable { symbol: char, coefficient: stored_coefficient, power: num})));
                            chars.next(); // take the ^ and nubmer off the string
                            for n in buffer.chars() {
                                chars.next();
                            }
                        },

                        Err(_) => output_vec.push(Type(Variable(Variable { symbol: char, coefficient: stored_coefficient, power: 1.0}))),
                    }
                },
                _ => {
                    output_vec.push(Type(Variable(Variable { symbol: char, coefficient: stored_coefficient, power: 1.0})));
                },
            };
            stored_coefficient = 1.0;
            buffer.clear();
        } else if char == '/' {
            output_vec.insert(0, Op(LeftParenthesis));  // "Onion" algorithm. adds layers of parenthesis
            output_vec.push(Op(RightParenthesis));      // until the order of operations checks out
            output_vec.push(wrap_buffer(char));

            match chars.clone().next() {
                Some('-') => {// if after an op the next char is a '-', assume that it is part of the next number
                    buffer += &chars.next().unwrap().to_string();
                },
                _ => {},
            }
        } else if char == '*' { // perform the same procedure for "*" so we simplify fully for multiplication
            output_vec.insert(0, Op(LeftParenthesis));  // "Onion" algorithm. adds layers of parenthesis
            output_vec.push(Op(RightParenthesis));      // until the order of operations checks out
            output_vec.push(wrap_buffer(char));

            match chars.clone().next() {
                Some('-') => {// if after an op the next char is a '-', assume that it is part of the next number
                    buffer += &chars.next().unwrap().to_string();
                },
                _ => {},
            }
        } else if is_operation(char) {
            output_vec.push(wrap_buffer(char));

            match chars.clone().next() {
                Some('-') if char != ')' && char != '(' => {// if after an op the next char is a '-', assume that it is part of the next number
                    buffer += &chars.next().unwrap().to_string();
                },
                _ => {},
            }
        } else {
            panic!("Invalid char, must be a num, variable, or operation");

        }

        peek = match chars.clone().next() {
            Some(num) => num,
            None => break,
        };//peek at next value
    }

    println!("output vec: {:?}", output_vec);
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
        '/' => Op(Divide),
        '^' => Op(Exponent),
        '-' => Op(Subtract),
        '(' => Op(LeftParenthesis),
        ')' => Op(RightParenthesis),
        _   => panic!(),
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
        vec![Type(Variable( Variable { symbol: 'x', coefficient: 24.0, power: 1.0})),
        Op(Multiply),
        Op(LeftParenthesis), Type(Float(32.03)), Op(Add), Type(Float(12.0)), Op(RightParenthesis)];


        assert_eq!(cmp_vec, out_vec);

    }

    #[test]
    fn division_with_variables() {
        let input = String::from("28x / 32.021");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis), Type(Variable(Variable { symbol: 'x', power: 1.0, coefficient: 28.0 })),
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
        Type(Variable(Variable { symbol: 'z', power: 75.0, coefficient: 1.0 })),
        Op(RightParenthesis),
        Op(Divide),
        Type(Float(2.0)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Variable(Variable { symbol: 'z', power: 2.0, coefficient: 1.0 })),
        ];

        assert_eq!(cmp_vec, out_vec);
    }

    #[test]
    fn single_digit_first_num_problem()
    {
        let input = String::from("3*2.3/46");
        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis),
        Type(Float(3.0)),
        Op(Multiply),
        Type(Float(2.3)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Float(46.0)),
        ];

        let out_vec = interpret(input);
        assert_eq!(cmp_vec, out_vec);
    }

    #[test]
    fn lots_of_negatives() {
        let input = String::from("-2.0/-4.0x^-1.0 - -4.0 * -5.0");
        let cmp_vec: Vec<ExpressionComponents> = vec![
        Op(LeftParenthesis),
        Type(Float(-2.0)),
        Op(RightParenthesis),
        Op(Divide),
        Type(Variable(Variable { symbol: 'x', power: -1.0, coefficient: -4.0 })),
        Op(Subtract),
        Type(Float(-4.0)),
        Op(Multiply),
        Type(Float(-5.0)),
        ];

        let out_vec = interpret(input);
        assert_eq!(cmp_vec, out_vec);
    }

}
