/*todo:
-fix all of the Err(()) enums-> figure out how to handle in evaluate_stack
-write more tests


*/
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Exponent,
    Multiply,
    Divide,
    Subtract,
    Add,
    LeftParenthesis,
    RightParenthesis,
}

impl Operand {
    #[allow(dead_code)]
    pub fn priority(&self) -> i32 {
        match *self {
            Operand::Exponent => 3,
            Operand::Multiply => 2,
            Operand::Divide => 2,
            Operand::Subtract => 1,
            Operand::Add => 1,
            Operand::LeftParenthesis => -1,
            Operand::RightParenthesis => -1,
        }
    }
}

#[allow(unused_imports)]
use Operand::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub symbol: char,
    pub power: f64,
    pub coefficient: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub values: Vec<Types>,
    pub operation: Operand,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Float(f64),
    Fraction(Fraction),
    Variable(Variable),
    Expression(Expression),
}
use Types::*;

pub trait Operations {
    // all of these methods want self, and another number, Fraction or var, and will return either Ok(T), where t is
    // number, Fraction or var, or a Err()
    fn add(num1: Self, num2: Types) -> Result<Types, ()>;

    fn sub(num1: Self, num2: Types) -> Result<Types, ()>;

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>;

    fn divide(num1: Self, num2: Types) -> Result<Types, ()>;

    // Literally just changes the sign
    fn negative(num1: Self) -> Self;
}

impl Operations for Fraction {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value + (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(Fraction {
                numerator: num1.numerator * value.denominator + value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Add,
                values: vec![Fraction(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value - (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(Fraction {
                numerator: num1.numerator * value.denominator - value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Subtract,
                values: vec![Fraction(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value * (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(Fraction {
                numerator: num1.numerator * value.numerator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Ok(Variable(Variable {
                symbol: value.symbol,
                power: value.power,
                coefficient: value.coefficient * num1.to_float()
            })),
            Expression(_) => Err(()),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1.to_float() / value)),
            Fraction(value) => Ok(Fraction(Fraction {
                numerator: num1.numerator * value.denominator,
                denominator: num1.denominator * value.numerator,
            })),
            Variable(value) => Ok(Variable(Variable {
                symbol: value.symbol,
                power: value.power * -1 as f64,
                coefficient: num1.to_float() / value.coefficient,
            })),
            Expression(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        Fraction {
            numerator: num1.numerator * -1,
            denominator: num1.denominator,
        }
    }
}

impl Operations for Variable {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(_) => Err(()),
            Fraction(_) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                Ok(Variable(Variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient + num1.coefficient,
                    power: value.power,
                }))
            } else {
                Ok(Expression(Expression {
                  operation: Operand::Add,
                  values: vec![Variable(num1), Variable(value)],
                }))
            },
            Expression(_) => Err(()),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(_) => Err(()),
            Fraction(_) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                Ok(Variable(Variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient - num1.coefficient,
                    power: value.power,
                }))
            } else {
                Ok(Expression(Expression {
                    operation: Operand::Subtract,
                    values: vec![Variable(num1), Variable(value)],
                }))
             },
             Expression(_) => Err(()),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Variable(Variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient * value,
                power: num1.power,
            })),
            Fraction(value) => Ok(Variable(Variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient * value.to_float(),
                power: num1.power,
            })),
            Variable(value) => if value.symbol == num1.symbol {
                if num1.power + value.power == 0.0 { //handle case where combined power is
                    return Ok(Float(value.coefficient * num1.coefficient));
                }
                Ok(Variable(Variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient * num1.coefficient,
                    power: num1.power + value.power,
                }))
            } else {
                Ok(Expression(Expression {
                    operation: Operand::Multiply,
                    values: vec![Variable(num1), Variable(value)],
                }))
            },
            Expression(_) => Err(()),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Variable(Variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient / value,
                power: num1.power,
            })),
            Fraction(value) => Ok(Variable(Variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient / value.to_float(),
                power: num1.power,
            })),
            Variable(value) => if value.symbol == num1.symbol {
                if num1.power - value.power == 0.0 { //handle case where power is 0
                    return Ok(Float(value.coefficient * num1.coefficient));
                }
                Ok(Variable(Variable {
                    symbol: value.symbol,
                    coefficient: num1.coefficient / value.coefficient,
                    power: num1.power - value.power,
                }))
            } else {
                Ok(Expression(Expression {
                    operation: Operand::Divide,
                    values: vec![Variable(num1), Variable(value)],
                }))
            },
            Expression(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        Variable {
            coefficient: num1.coefficient * -1.0,
            power: num1.power,
            symbol: num1.symbol,
        }
    }
}

impl Operations for f64 {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 + value)),
            Fraction(value) => Ok(Float(num1 + value.to_float())),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Add,
                values: vec![Float(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 - value)),
            Fraction(value) => Ok(Float(num1 - value.to_float())),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Subtract,
                values: vec![Float(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>{
        match num2 {
            Float(value) => Ok(Float(num1 * value)),
            Fraction(value) => Ok(Float(num1 * value.to_float())),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Multiply,
                values: vec![Float(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 / value)),
            Fraction(value) => Ok(Float(num1 / value.to_float())),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Divide,
                values: vec![Float(num1), Variable(value)],
            })),
            Expression(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        -num1
    }
}

impl Operations for Expression {
    #[allow(unused_variables)]
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        Err(())
    }
    #[allow(unused_variables)]
    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        Err(())

    }
    #[allow(unused_variables)]
    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>{
        Err(())
    }
    #[allow(unused_variables)]
    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        Err(())
    }
    #[allow(unused_variables)]
    fn negative(num1: Self) -> Self {
        num1
    }
}

/*
impl Operations for Inoperable {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {

        // iterate through the values in the values vector
        for num in num1.values.iter() {
            match add(num, num2) {
                Inoperable(_) => continue, // if the operation would return an Inoperable, we dont want to do it
                _ => { // passes the first test, but now we need to check other things

                    //check previous operation (from operations vector)

                    // check next operation

                    // if both checks pass, we can actually perform the add operation

                    //perform the operation, and make a new Inoperable struct to return

                } //is this just complete lunacy or does it look kinda reasonable to you guys?
            }
        }

        // if all of that failed, then we need to instead return a new Inoperable struct, with the current
        // add option appended to it (or should we prepend it to the inoperable struct?)


        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        todo!();
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>{
        todo!();
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        todo!();
    }

    fn negative(num1: Self) -> Self {
        todo!();
    }
}
*/


impl Fraction {
    fn to_float(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    #[allow(dead_code)]
    fn simplify(self) -> Fraction {
        let gcd = gcd(self.numerator, self.denominator);

        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd
        }
    }
}

//using Euclidean algorithm
#[allow(dead_code)]
fn gcd(num1: i64, num2: i64) -> i64 {
    let order: (i64, i64) = match num1.cmp(&num2) { //sort the pair of values into a tuple
        Ordering::Greater => (num1, num2),
        Ordering::Less => (num2, num1),
        Ordering::Equal => return num1, //or return one of them if equal
    };

    if order.1 == 0 {
        return order.0;
    }

    gcd(order.0 % order.1, order.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Variable Type Tests Start */
/*    #[test]
    fn adding_variables_same_power() {
        let var1 = Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        };
        let var2 = Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 4.0,
        };

        let var3 = match Variable::add(var1, Variable(var2)) {
            Ok(Variable(some)) => some,
            _ => panic!(),
        };
    Expression(Inoperable), //unsure on the functionality of this test piece, maybe I deleted a line?
// yea this means nothing by itself. maybe we should just delete it, and fix the test later?
        assert_eq!(var3.coefficient, 5.0);
    }*/

    #[test]
    fn adding_variables_different_power() {
        let var1 = Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        };
        let var2 = Variable {
            symbol: 'x',
            power: 2.0,
            coefficient: 4.0,
        };

        assert_eq!(Ok(Expression(Expression{
            operation: Operand::Add,
            values: vec![Variable(var1.clone()), Variable(var2.clone())]
        })), Variable::add(var1, Variable(var2)));
    }

    #[test]
    fn add_variables_to_fraction() {
        let var = Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = Fraction {
            numerator: 4,
            denominator: 5,
        };

        assert_eq!(Err(()), Variable::add(var, Fraction(frac)));
    }

    #[test]
    fn multiply_variable_and_fraction() {
        let var = Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = Fraction {
            numerator: 7,
            denominator: 4,
        };

        let value = Variable::multiply(var, Fraction(frac));

        assert_eq!(value, Ok(Variable(Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 7.0
        })));
    }

    #[test]
    fn divide_variable_by_fraction(){
        let var = Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = Fraction {
            numerator: 8,
            denominator: 4,
        };

        let value = Variable::divide(var, Fraction(frac));

        assert_eq!(value, Ok(Variable(Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 2.0
        })));
    }

    #[test]
    fn divide_variable_by_variable(){
        let var1 = Variable {
            symbol: 'y',
            power: 3.0,
            coefficient: 5.0,
        };

        let var2 = Variable{
            symbol: 'y',
            power: 2.0,
            coefficient: 2.0,
        };

        let value = Variable::divide(var1, Variable(var2));

        assert_eq!(value, Ok(Variable(Variable {
            symbol: 'y',
            power: 1.0,
            coefficient: 2.5
        })));
    }

    #[test]
    fn divide_variable_by_multiplied_variable(){
        let var1 = Variable {
            symbol: 'y',
            power: 3.0,
            coefficient: 6.0,
        };

        let var2 = Variable{
            symbol: 'y',
            power: 2.0,
            coefficient: 2.0,
        };

        let var3 = Variable{
            symbol: 'y',
            power: 5.0,
            coefficient: 6.0,
        };

        let value = Variable::divide(var1,
            Variable::multiply(var2, Variable(var3)).unwrap()
        );

        assert_eq!(value, Ok(Variable(Variable {
            symbol: 'y',
            power: -4.0,
            coefficient: 0.5
        })));
    }

    /* Variable Tests End */

    /* Fraction Tests Start */
    #[test]
    fn divide_fraction_by_variable(){
        let var = Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = Fraction {
            numerator: 7,
            denominator: 4,
        };

        let value = Fraction::divide(frac, Variable(var));

        assert_eq!(value, Ok(Variable(Variable {
            symbol: 'y',
            power: -2.0,
            coefficient: 0.4375
        })));
    }

    #[test]
    fn simplify_fraction(){
        let frac = Fraction {
            numerator: 4,
            denominator: 12
        };

        let value = frac.simplify();

        assert_eq!(value, Fraction{
            numerator: 1,
            denominator: 3
        });
    }

    /* Fraction Tests End */
    #[test]
    fn find_gcd(){
        let num1 = 1670;
        let num2 = 560;

        let value = gcd(num1, num2);

        assert_eq!(value, 10);
    }
}
