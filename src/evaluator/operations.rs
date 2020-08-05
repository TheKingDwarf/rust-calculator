use super::numbers::*;

//sqrt function
use num_integer::Roots;

use std::cmp::Ordering;

pub trait Operations {
    // all of these methods want self, and another number, Fraction or var, and will return either Ok(T), where t is
    // number, Fraction or var, or a Err()
    fn add(num1: Self, num2: Types) -> Result<Types, ()>;

    fn sub(num1: Self, num2: Types) -> Result<Types, ()>;

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>;

    fn divide(num1: Self, num2: Types) -> Result<Types, ()>;

    fn exponentiate(num1: Self, num2: Types) -> Result<Types, ()>;

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
                power: value.power * -1.0,
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

    fn exponentiate(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => if value.abs() < 1.0 && //if the exponent is a root
                            (num1.numerator < 0 || num1.denominator < 0) { //and the fraction is negative
                Ok(Expression(Expression { //return as expression to avoid taking roots of negative
                    values: vec![Fraction(num1), Float(value)],
                    operation: Operand::Exponent,
                }))
            } else {
                match value.partial_cmp(&0.0).unwrap() {
                    Ordering::Greater   =>
                    Ok(Fraction(Fraction {
                        numerator: (num1.numerator as f64).powf(value) as i64,
                        denominator: (num1.denominator as f64).powf(value) as i64,
                    })),
                    Ordering::Less      =>
                    Ok(Fraction(Fraction {
                        numerator: (num1.denominator as f64).powf(value) as i64,
                        denominator: (num1.numerator as f64).powf(value) as i64,
                    })),
                    Ordering::Equal     =>
                    Ok(Float(1.0)),
                }
            },
            //cloning the Types variant here because a reference would have to be explicit to each type, so the function would need four overloads
            Fraction(value) => if value.numerator > 0 && Fraction::exponentiates_to_integer(&num1, Fraction(value.clone())) {
                let float_value = value.clone().to_float();
                Ok(Fraction(Fraction {
                    numerator: (num1.numerator as f64).powf(float_value) as i64,
                    denominator: (num1.denominator as f64).powf(float_value) as i64,
                }))
            } else {
                Ok(Expression(Expression {
                    values: vec![Fraction(num1), Fraction(value)],
                    operation: Operand::Exponent,
                }))
            },
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Exponent,
                values: vec![Fraction(num1), Variable(value)],
            })),
            Expression(value) => Err(())
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
                    return Ok(Float(num1.coefficient / value.coefficient));
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

    fn exponentiate(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => {
                if value != 0.0 {
                    Ok(Variable(Variable {
                        symbol: num1.symbol,
                        power: num1.power * value,
                        coefficient: num1.coefficient.powf(value),
                    }))
                } else {
                    Ok(Float(num1.coefficient))
                }
            },
            //TODO just leaving as an expression for now but would be simpler to make the power attribute
            //of the Variable struct a Types variant and leave as a fraction to return a Variable -oisin
            Fraction(value) => Ok(Expression(Expression {
                operation: Operand::Exponent,
                values: vec![Variable(num1), Fraction(value)],
            })),
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Exponent,
                values: vec![Variable(num1), Variable(value)],
            })),
            Expression(value) => Err(())
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
            Variable(value) => Ok(Variable(Variable {
                symbol: value.symbol,
                coefficient: value.coefficient * num1,
                power: value.power,
            })),
            Expression(_) => Err(()),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => {
                if (num1 / value).fract() == 0.0 { // if dividing gives an int
                    Ok(Float(num1 / value))
                } else if num1.fract() == 0.0 && value.fract() == 0.0 { //convert to fraction if possible
                    Ok(Fraction(Fraction {
                        numerator: num1 as i64,
                        denominator: value as i64,
                    }))
                } else {
                    Ok(Float(num1 / value))
                }
            },
            Fraction(value) => Ok(Float(num1 / value.to_float())),
            Variable(value) => Ok(Variable(Variable{
                    symbol: value.symbol,
                    power: -1.0 * value.power,
                    coefficient: num1 / value.coefficient
                })),
            Expression(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        -num1
    }

    fn exponentiate(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => if value > 0.0 || value.abs().fract() == 0.0 { //ensure positive or not a root
                match value.partial_cmp(&0.0).unwrap() {
                    Ordering::Greater 	=> Ok(Float(num1.powf(value))),
                    Ordering::Less		=> Ok(Float(1.0 / num1.powf(value))), //flip if power < 0
                    Ordering::Equal		=> Ok(Float(1.0)),
                }
            } else {
                Ok(Expression(Expression {
                    operation: Operand::Exponent,
                    values: vec![Float(num1), num2],
                }))
            },
            Fraction(value) => {
                if Fraction::exponentiates_to_integer(&value, Float(num1.clone())) {//checking here if the float evaluates evenly with a fractiional exponent, if it does converting to i64 to comply with nth_root function
                    match value.clone().to_float().partial_cmp(&0.0).unwrap() {
                        Ordering::Greater 	=> Ok(Float(num1.powf(value.clone().to_float()))),
            			Ordering::Less		=> Ok(Float(1.0 / num1.powf(value.clone().to_float()))), //flip if power < 0
            			Ordering::Equal		=> Ok(Float(1.0)),
            		}
                } else {
                    Ok(Expression(Expression {
                        operation: Operand::Exponent,
                        values: vec![Float(num1), Fraction(value)],
                    }))
                }
            },
            Variable(value) => Ok(Expression(Expression {
                operation: Operand::Exponent,
                values: vec![Float(num1), Variable(value)],
            })),
            Expression(value) => Err(()),
        }
    }
}

impl Operations for Expression {
    #[allow(unused_variables)]
    fn exponentiate(num1: Self, num2: Types) -> Result<Types, ()> {
        Err(())
    }

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
        // distributive property
        if num1.operation == Add || num1.operation == Subtract {
            // evaluate each value seperately
            let returned = (
                crate::evaluator::evaluate_expression(
                    Expression {
                        values: vec!(num1.values[0].clone(), num2.clone()),
                        operation: Multiply,
                    }
                ),
                crate::evaluator::evaluate_expression(
                    Expression {
                        values: vec!(num1.values[1].clone(), num2.clone()),
                        operation: Multiply,
                    }
                ),
            );

            return Ok(Expression( Expression{
                values: vec!(returned.0, returned.1),
                operation: num1.operation
            }));
        }

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

    //checks if a fraction exponentiates a number without a fractional component in the return type
    fn exponentiates_to_integer(fraction: &Fraction, other: Types) -> bool {
        match other {
            Float(value) => if (value.powf(fraction.numerator as f64).fract() == 0.0)
                            && (value.powf(fraction.denominator as f64).fract() == 0.0) {
                true
            } else {
                false
            },
            Fraction(value) => {
                let float_value = value.clone().to_float();
                println!("Float_value: {}, numerator: {}, denominator: {}",
                float_value, (fraction.numerator as f64).powf(float_value),
                (fraction.denominator as f64).powf(float_value));

                if (fraction.numerator as f64).powf(float_value).fract() == 0.0 // TODO:
                && (fraction.denominator as f64).powf(float_value).fract() == 0.0 {
                    true
                } else {
                    false
                }
            },
            Variable(_value) => false,
            Expression(_value) => false,
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

    #[test]
    fn cube_variable_w_coefficient(){
        let var = Variable{
            symbol: 'y',
            power: 2.0,
            coefficient: 3.0,
        };

        let value = Variable::exponentiate(var, Float(3.0));

        assert_eq!(Ok(Variable(Variable{
            symbol: 'y',
            power: 6.0,
            coefficient: 27.0,
        })), value);
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

    #[test]
    fn square_fraction(){
        let frac = Fraction {
            numerator: 3,
            denominator:4
        };

        let value = Fraction::exponentiate(frac, Float(2.0));

        assert_eq!(Ok(Fraction(Fraction{
            numerator: 9,
            denominator: 16
        })), value);
    }

    #[test]
    fn fraction_to_zero(){
        let frac = Fraction {
            numerator: 3,
            denominator:4
        };

        let value = Fraction::exponentiate(frac, Float(0.0));

        assert_eq!(Ok(Float(1.0)), value);
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
