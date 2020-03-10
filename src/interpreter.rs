use crate::evaluator::numbers::*;

pub fn interpret(input: String) -> Vec<ExpressionComponents>{
    let mut output_vec: Vec<ExpressionComponents> = vec![];

    let mut chars = input.chars();

    //buffer used to store single number
    let mut buffer = "".to_string();

    let mut peek = chars.clone().next().unwrap(); // peek at the next value

    //make input iterable by char.
    while let Some(char) = chars.next() {
        if char.is_digit(10) {
            buffer = char.to_string(); // puts the current char onto the buffer
            while peek.is_digit(10) || peek == '.' { //loop through to get full number
                //todo negative numbers in interpreter. Currently broken
                buffer = format!("{}{}", buffer, chars.next().unwrap());

                peek = match chars.clone().next() {
                    Some(num) => num,
                    None => break,
                };//peek at next value
            }

            match buffer.parse::<f64>() { //push the current number with all digits to the vector
                Ok(num) => {
                    output_vec.push(Type(Float(num)));
                    buffer.clear();
                },
                Err(_) => panic!("Failed to parse buffer: *{}* ", buffer)//TODO error handling as well
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

}
