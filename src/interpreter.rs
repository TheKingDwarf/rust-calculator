use crate::evaluator::numbers::*;

pub fn interpret(input: String) -> Vec<ExpressionComponents>{
    let mut output_vec: Vec<ExpressionComponents> = vec![];

    //make input iterable by char.
    for char in input.iter().peekable() {
        if char.is_digit() {
            //buffer used to store single number
            let mut buffer = "";

            while (*input.peek().is_digit() || *input.peek() == ".") { //loop through to get full number
                buffer = format!("{}{}", buffer, input.next());
            }

            match buffer.parse::<f64>() { //push the current number with all digits to the vector
                Ok(num) => output_vec.push(Type(Float(num))),
                Err(_) => panic!("Failed to parse buffer")//TODO error handling as well
            }


            if *input.peek().is_alphabetic() || *input.peek() == "(" { // if the next char is a var or (, insert a multiplication in between
                output_vec.push("*".wrap_buffer());
            }

        } else if char.is_alphabetic() {
            output_vec.push(Type(Variable(Variable { symbol: char, coefficient: 1, power: 1})));

        } else if char.is_operation() {
            output_vec.push(char.wrap_buffer());

        } else {
            panic!("Invalid char, must be a num, variable, or operation");

        }
    }

    output_vec
}//end of interpret

impl String {
    pub fn is_operation(&self) -> bool {
        match self {
            "*" || "+" || "/" || "^" ||
            "-" || "(" || ")" => true,
            _ => false,
        }
    }
    pub fn wrap_buffer(&self) -> ExpressionComponents {
        match self {
            "*" => Op(Multiply) ,
            "+" => Op(Add),
            "/" => Op(Divide),
            "^" => Op(Exponent),
            "-" => Op(Subtract),
            "(" => Op(LeftParenthesis),
            ")" => Op(RightParenthesis),
        }
    }// end of wrap_buffer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multidigit_numbers() {

        let input = String::from("28*32.021"); //lets just test the decimal while we're at it

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![Type(Float(28)), Op(Multiply), Type(Float(32.021))];

        assert_eq!(out_vec, cmp_vec);

    }

    #[test]
    fn variables_with_parenthesis() {

        let input = String::from("24x * (32.03 + 12)");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> =
        vec![Type(Float(24)), Op(Multiply),
        Type(Variable( Variable { symbol: 'x', coefficient: 1, power: 1})),
        Op(LeftParenthesis), Type(Float(32.03)), Op(Add), Type(Float(12)), Op(RightParenthesis))];


        assert_eq!(out_vec, cmp_vec);

    }

    #[test]
    fn division_with_variables() {
        let input = String::from("28x / 32.021");

        let out_vec = interpret(input);

        let cmp_vec: Vec<ExpressionComponents> = vec![Type(Float(28)), Op(Multiply), Type(Variable( Variable { symbol: 'x', coefficient: 1, power: 1})), Op(Divide), Type(Float(32.021))];

        assert_eq!(out_vec, cmp_vec);

//more tests? are these passing?
// is the program compiling with the enums being moved? or are my use statements not correct
//It is giving import errors from mod.rs
    }

}


/*
