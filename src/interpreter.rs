use crate::evaluator::numbers::*;

pub fn interpret(input: String) -> Vec<ExpressionComponents>{
    let mut output_vec: Vec<ExpressionComponents> = vec![];

    let mut peekable = input.chars().peekable();
    //make input iterable by char.
    for char in peekable.clone() {
        if char.is_digit(10) {
            //buffer used to store single number
            let buffer = "";

            while peekable.peek().unwrap().is_digit(10) || *peekable.peek().unwrap() == '.' { //loop through to get full number
                let buffer = format!("{}{}", buffer, peekable.next().unwrap());
            }

            match buffer.parse::<f64>() { //push the current number with all digits to the vector
                Ok(num) => output_vec.push(Type(Float(num))),
                Err(_) => panic!("Failed to parse buffer")//TODO error handling as well
            }


            if peekable.peek().unwrap().is_alphabetic() || *peekable.peek().unwrap() == '(' { // if the next char is a var or (, insert a multiplication in between
                output_vec.push(wrap_buffer('*'));
            }

        } else if char.is_alphabetic() {
            output_vec.push(Type(Variable(Variable { symbol: char, coefficient: 1.0, power: 1.0})));

        } else if is_operation(char) {
            output_vec.push(wrap_buffer(char));

        } else {
            panic!("Invalid char, must be a num, variable, or operation");

        }
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
        Op(LeftParenthesis), Type(Float(32.03)), Op(Add), Type(Float(12.0)), Op(RightParenthesis)];


        assert_eq!(out_vec, cmp_vec);

    }

    #[test]
    fn division_with_variables() {
        let input = String::from("28x / 32.021");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![Type(Float(28.0)), Op(Multiply), Type(Variable( Variable { symbol: 'x', coefficient: 1.0, power: 1.0})), Op(Divide), Type(Float(32.021))];

        assert_eq!(out_vec, cmp_vec);

//more tests? are these passing?
// is the program compiling with the enums being moved? or are my use statements not correct
//It is giving import errors from mod.rs
    }
}
