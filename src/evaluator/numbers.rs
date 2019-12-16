/*todo:
-fix all of the Err(()) enums-> figure out how to handle in evaluate_stack
-write more tests


*/
#[derive(Debug)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub struct fraction {
    numerator: i64,
    denominator: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub struct variable {
    symbol: char,
    power: f64,
    coefficient: f64,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Types {
    Float(f64),
    Fraction(fraction),
    Variable(variable),
}
// name space the type enum so that we dont have to prepend each case in our match statements with Types::
use Types::*;

trait Operations {
    // all of these methods want self, and another number, fraction or var, and will return either Ok(T), where t is
    // number, fraction or var, or a Err()
    fn add(num1: Self, num2: Types) -> Result<Types, ()>;

    fn sub(num1: Self, num2: Types) -> Result<Types, ()>;

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>;

    fn divide(num1: Self, num2: Types) -> Result<Types, ()>;

    // Literally just changes the sign
    fn negative(num1: Self) -> Self;
}

impl Operations for fraction {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value + (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(fraction {
                numerator: num1.numerator * value.denominator + value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Err(())
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value - (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(fraction {
                numerator: num1.numerator * value.denominator - value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Err(())
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value * (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(fraction {
                numerator: num1.numerator * value.numerator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(value) => Ok(Variable(variable {
                symbol: value.symbol,
                power: value.power,
                coefficient: value.coefficient * num1.to_float();
            })),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1.to_float() / value)),
            Fraction(value) => Ok(Fraction(fraction {
                numerator: num1.numerator * value.denominator,
                denominator: num1.denominator * value.numerator,
            })),
            Variable(value) => Ok(Variable(variable {
                symbol: value.symbol,
                power: value.power * -1 as f64,
                coefficient: num1.to_float() / value.coefficient,
            })),
        }
    }

    fn negative(num1: Self) -> Self {
        fraction {
            numerator: num1.numerator * -1,
            denominator: num1.denominator,
        }
    }
}

impl Operations for variable {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Err(()),
            Fraction(value) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                Ok(Variable(variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient + num1.coefficient,
                    power: value.power,
                }))
            } else {
                 Err(())
            },
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Err(()),
            Fraction(value) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                Ok(Variable(variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient - num1.coefficient,
                    power: value.power,
                }))
            } else {
                 Err(())
             },
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Variable(variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient * value,
                power: num1.power,
            })),
            Fraction(value) => Ok(Variable(variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient * value.to_float(),
                power: num1.power,
            })),
            Variable(value) => if value.symbol == num1.symbol {
                Ok(Variable(variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient * num1.coefficient,
                    power: num1.power + value.power,
                }))
            } else {
                Err(())
            },
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Variable(variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient / value,
                power: num1.power,
            })),
            Fraction(value) => Ok(Variable(variable {
                symbol: num1.symbol,
                coefficient: value.to_float(),
                power: num1.power,
            })),
            Variable(value) => if value.symbol == num1.symbol {
                Ok(Variable(variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient / num1.coefficient,
                    power: num1.power - value.power,
                }))
            } else {
                Err( () )
            }
        }
    }

    fn negative(num1: Self) -> Self {
        variable {
            coefficient: num1.coefficient * -1.0,
            power: num1.power,
            symbol: num1.symbol,
        }
    } //main doesn't mod evaluator anywhere so that might have something to do with it
}

impl Operations for f64 {
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 + value)),
            Fraction(value) => Ok(Float(num1 + value.to_float()),
            Variable(value) => Err( () ),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 - value)),
            Fraction(value) => Ok(Float(num1 - value.to_float()),
            Variable(value) => Err(() ),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>{
        match num2 {
            Float(value) => Ok(Float(num1 * value)),
            Fraction(value) => Ok(Float(num1 * value.to_float())),
            Variable(_) => Err(()),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 / value)),
            Fraction(value) => Ok(Float(num1 / value.to_float()),
            Variable(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        -num1
    }
}

impl fraction {
    fn to_float(self) -> f64 {
        self.numerator as f64 / self.denominator as f64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_variables_same_power() {
        let var1 = variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        };
        let var2 = variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 4.0,
        };

        let var3 = match variable::add(var1, Variable(var2)) {
            Ok(Variable(some)) => some,
            _ => panic!(),
        };

        assert_eq!(var3.coefficient, 5.0);
    }

    #[test]
    fn adding_variables_different_power() {
        let var1 = variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        };
        let var2 = variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        assert_eq!(Err(()), variable::add(var1, Variable(var2)));
    }

    #[test]
    fn add_variables_to_fraction() {
        let var = variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = fraction {
            numerator: 4,
            denominator: 5,
        };

        assert_eq!(Err(()), variable::add(var, Fraction(frac)));
    }

    #[test]
    fn multiply_variable_and_fraction() {
        let var = variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = fraction {
            numerator: 7,
            denominator: 4,
        };

        let value = variable::multiply(var, Fraction(frac));

        assert_eq!(value, Ok(Variable(variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 7.0
        })));
    }
}
