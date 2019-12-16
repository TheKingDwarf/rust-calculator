/*todo:
-fix all of the Err(()) enums-> figure out how to handle in evaluate_stack
-write more tests


*/

struct fraction {
    numerator: i64,
    denominator: i64,
}

struct variable {
    symbol: char,
    power: f64,
    coefficient: f64,
}

enum types {
    Float(f64),
    Fraction(fraction),
    Variable(variable),
}

trait operations {
    // all of these methods want self, and another number, fraction or var, and will return either Ok(T), where t is
    // number, fraction or var, or a Err()
    fn add<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())>;

    fn sub<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())>;

    fn multiply<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())>;

    fn divide<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())>;

    // Literally just changes the sign
    fn negative(num1: Self) -> Self;
}

impl operations for fraction {
    fn add<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(value + (num1.numerator/num1.denominator) as f64,
            Fraction(value) => Ok(fraction {
                numerator: num1.numerator * value.denominator + value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            }),
            Variable(value) => Err()
        }
    }

    fn sub<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(value - (num1.numerator/num1.denominator) as f64,
            Fraction(value) => Ok(fraction {
                numerator: num1.numerator * value.denominator - value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            }),
            Variable(value) => Err(())
        }
    }

    fn multiply<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(value * (num1.numerator/num1.denominator) as f64,
            Fraction(value) => Ok(fraction {
                numerator: num1.numerator * value.numerator,
                denominator: num1.denominator * value.denominator,
            }),
            Variable(value) => Ok(variable {
                symbol: value.symbol,
                power: value.power,
                coefficient: value.coefficient * (num1.numerator/num1.denominator) as f64
            }),
        }
    }

    fn divide<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(value * (num1.denominator/num1.numerator) as f64,
            Fraction(value) => Ok(fraction {
                numerator: num1.numerator * value.denominator,
                denominator: num1.denominator * value.numerator,
            }),
            Variable(value) => Ok(variable {
                symbol: value.symbol,
                power: value.power * -1,
                coefficient: value.coefficient * (num1.denominator/num1.numerator) as f64
            }),
        }
    }

    fn negative(num1: Self) -> Self {
        fraction {
            numerator: num1.numerator * -1,
            denominator: num1.denominator,
        }
    }
}

impl operations for variable {
    fn add<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Err(()),
            Fraction(value) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient + num1.coefficient,
                    power: value.power,
                }
            } else {
                 Err(())
            },
        }
    }

    fn sub<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Err(()),
            Fraction(value) => Err(()),
            Variable(value) => if value.symbol == num1.symbol && value.power == num1.power {
                variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient - num1.coefficient,
                    power: value.power,
                }
            } else {
                 Err(()),
            }
        }
    }

    fn multiply<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient * value,
                power: num1.power,
            }),
            Fraction(value) => Ok( variable {
                symbol: num1.symbol,
                coefficient: (num2.numerator / num2.denominator) as float,
                power: num1.power,
            },
            Variable(value) => if value.symbol == num1.symbol {
                variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient * num1.coefficient,
                    power: num1.power + value.power,
                }
            } else {
                Err(()),
            }
        }
    }

    fn divide<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(variable {
                symbol: num1.symbol,
                coefficient: num1.coefficient / value,
                power: num1.power,
            }),
            Fraction(value) => Ok( variable {
                symbol: num1.symbol,
                coefficient: (num2.numerator / num2.denominator) as float,
                power: num1.power,
            },
            Variable(value) => if value.symbol == num1.symbol {
                variable {
                    symbol: value.symbol,
                    coefficient: value.coefficient / num1.coefficient,
                    power: num1.power - value.power,
                }
            } else {
                Err( () ),
            }
        }
    }

    fn negative(num1: Self) -> Self {
        variable {
            coefficient: num1.coefficient *-1,
            power: num1.power,
            symbol: num1.symbol,
        }
    }

}

impl operations for f64 {
    fn add<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ()> {
        match num2 {
            Float(value) => Ok(num2 + value),
            Fraction(value) => Ok( num1 + (value.numerator / value.denominator) ) as float,
            Variable(value) => Err( () ),
        }
    }

    fn sub<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())> {
        match num2 {
            Float(value) => Ok(num1 - value),
            Fraction(value) => Ok( num1 - (value.numerator / value.denominator) ) as float,
            Variable(value) => Err(() ),
        }
    }

    fn multiply<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ())>{
        match num2 {
            Float(value) => Ok(num1 * value),
            Fraction(value) => Ok( num1 * (value.numerator / value.denominator) ) as float,
            Variable(value) => Err( () ),
        }
    }

    fn divide<T: operations>(num1: Self, num2: types(T)) -> Result<Box<dyn operations>, ()> {
        match num2 {
            Float(value) => Ok(num1 / value),
            Fraction(value) => Ok( num1 / (value.numerator / value.denominator) ) as float,
            Variable(value) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        num1 * -1
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

        let var3 = add(var1, var2).unwrap();

        assert_eq!(var3.coefficient, 5);
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

        let var3 = add(var1, var2).unwrap();

        assert_ne!(var3.coefficient, 5);
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

        match add(var, frac) {
            Ok(_) => panic!("Should've returned error!"),
            Err(value) => assert_eq!(value, vec![var, frac]),
        };
    }

    #[test]
    fn multiply_variable_and_fraction() {
        let var = variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        let frac = fraction {
            numerator: 5,
            denominator: 4,
        };

        let value = multiply(var, frac).unwrap();

        assert_eq!(value, variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 5.0
        });
    }
}
