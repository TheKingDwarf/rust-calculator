use std::io;
use crate::evaluator::numbers::*;

pub fn get_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input Error");
    input
}

pub fn format_expression(input: Vec<ExpressionComponents>) -> String {
    let mut output = String::new();
    for val in input {
        match val {
            Type(Float(t)) => output += &t.to_string(),
            Type(Fraction(t)) => {
                output += &format!("({}/{})", t.numerator, t.denominator);
            },
            Type(Variable(t)) => {
                if t.coefficient != 1.0 {
                    output += &format!("{}", t.coefficient);
                }

                output += &format!("{}", &t.symbol);

                if t.power != 1.0 {
                    output += &format!("^{}", &t.power);
                }
            },
            Type(Expression(t)) => {
                let mut type_vec = t.values.clone();

                output += &format_expression(
                    vec!(
                        Type(type_vec.pop().unwrap()),
                        Op(t.operation),
                        Type(type_vec.pop().unwrap()),
                    )
                )
            }
            Op(LeftParenthesis)  => output += "(",
            Op(RightParenthesis) => output += ")",
            Op(Exponent) => output += "^",
            Op(Multiply) => output += " * ", // lets move to main.rs and implement this there to make sure it works
            Op(Divide) => output += " / ", // could also be worth adding some tests to io.rs
            Op(Subtract) => output += " - ",
            Op(Add) => output += " + ",

        };
    }
// actually, do we want to add tests here or to main?
// here we could just copy the vectors we were using from the interpreter and use them as the input to the above function

// i think the tests should be here, and i think it would be a good idea to create them as you just described
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_variables() {
        let in_vec = vec![
        Type(Variable(Variable{
            symbol: 'x',
            power: 3.0,
            coefficient: 2.55
        })),
        Op(Divide),
        Type(Variable(Variable{
            symbol: 'y',
            power: 1.0,
            coefficient: 5.0
        }))];

        assert_eq!(format_expression(in_vec), "2.55x^3 / 5y");
    }

    #[test]
    fn formatting_expressions() {
        let in_vec = vec![
        Type(Expression(Expression {
            values: vec![Variable(Variable {
                symbol: 'x',
                power: 1.0,
                coefficient: 1.0 }), Float(9.0)],
            operation: Exponent })
        ),
        Op(Multiply),
        Type(Variable(Variable{
            symbol: 'x',
            power: 55.0,
            coefficient: 1.0
        }))];

        assert_eq!(format_expression(in_vec),
            "9^x * x^55"
        );
    }
}
