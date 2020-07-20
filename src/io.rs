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

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]

}
