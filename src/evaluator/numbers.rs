use std::cmp::Ordering;
use std::cell::Cell;

#[derive(Debug)]
#[derive(PartialEq)]
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
    pub fn priority(&self) -> i32 {
        match *self {
            Operand::Exponent => 3,
            Operand::Multiply => 2,
            Operand::Divide => 2,
            Operand::Subtract => 1,
            Operand::Add => 1,
            Operand::LeftParenthesis => 4,
            Operand::RightParenthesis => 4,
        }
    }
}

use Operand::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Variable {
    symbol: char,
    power: f64,
    coefficient: f64,
}

//unfortunately Debug and PartialEq are not compatible with Cell
//this will likely screw up our tests later on but for now interior mutability is
//an alternative to making the
//#[derive(Debug)]
//#[derive(PartialEq)]
pub struct Inoperable {
    values: Cell<Vec<Types>>,
    operations: Cell<Vec<Operand>>,
}

//#[derive(Debug)]
//#[derive(PartialEq)]
pub enum Types {
    Float(f64),
    Fraction(Fraction),
    Variable(Variable),
    Inoperable(Inoperable),
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
            Variable(_) => Err(()),
            Inoperable(_) => Err(()),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(value - (num1.numerator/num1.denominator) as f64)),
            Fraction(value) => Ok(Fraction(Fraction {
                numerator: num1.numerator * value.denominator - value.numerator * num1.denominator,
                denominator: num1.denominator * value.denominator,
            })),
            Variable(_) => Err(()),
            Inoperable(_) => Err(()),
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
            Inoperable(_) => Err(()),
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
            Inoperable(_) => Err(()),
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
                 Err(())
            },
            Inoperable(_) => Err(()),
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
                 Err(())
             },
             Inoperable(_) => Err(()),
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
                Err(())
            },
            Inoperable(_) => Err(()),
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
                Err( () )
            },
            Inoperable(_) => Err(()),
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
            Variable(_) => Err( () ),
            Inoperable(_) => Err(()),
        }
    }


    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 - value)),
            Fraction(value) => Ok(Float(num1 - value.to_float())),
            Variable(_) => Err( () ),
            Inoperable(_) => Err(()),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()>{
        match num2 {
            Float(value) => Ok(Float(num1 * value)),
            Fraction(value) => Ok(Float(num1 * value.to_float())),
            Variable(_) => Err(()),
            Inoperable(_) => Err(()),

        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        match num2 {
            Float(value) => Ok(Float(num1 / value)),
            Fraction(value) => Ok(Float(num1 / value.to_float())),
            Variable(_) => Err( () ),
            Inoperable(_) => Err(()),
        }
    }

    fn negative(num1: Self) -> Self {
        -num1
    }
}

impl Operations for Inoperable {

//here we have a problem because we cannot declare the vector inside of the struct
//to always be mutable unless the struct itself is mutable- mutability is inherited
    fn add(num1: Self, num2: Types) -> Result<Types, ()> {

        //janky implementation for messing with the vectors inside of Cells
        num1.add_operand(Operand::Add);

        match num2 {
            Float(value) => num1.add_value(Float(value)),
            Fraction(value) => num1.add_value(Fraction(value)),
            Variable(value) => num1.add_value(Variable(value)),
            Inoperable(value) => num1.add_value(Inoperable(value)),
        }
    }

    fn sub(num1: Self, num2: Types) -> Result<Types, ()> {

        num1.add_operand(Operand::Subtract);

        match num2 {
            Float(value) => num1.add_value(Float(value)),
            Fraction(value) => num1.add_value(Fraction(value)),
            Variable(value) => num1.add_value(Variable(value)),
            Inoperable(value) => num1.add_value(Inoperable(value)),
        }
    }

    fn multiply(num1: Self, num2: Types) -> Result<Types, ()> {

        num1.add_operand(Operand::Multiply);

        match num2 {
            Float(value) => num1.add_value(Float(value)),
            Fraction(value) => num1.add_value(Fraction(value)),
            Variable(value) => num1.add_value(Variable(value)),
            Inoperable(value) => num1.add_value(Inoperable(value)),
        }
    }

    fn divide(num1: Self, num2: Types) -> Result<Types, ()> {
        num1.add_operand(Operand::Divide);

        match num2 {
            Float(value) => num1.add_value(Float(value)),
            Fraction(value) => num1.add_value(Fraction(value)),
            Variable(value) => num1.add_value(Variable(value)),
            Inoperable(value) => num1.add_value(Inoperable(value)),
        }
    }

    fn negative(num1: Self) -> Self {
        num1
    }
}

impl Inoperable {

    //janky way of modifying the vector
    fn add_operand(&self, operand: Operand) {
        let mut ops = self.operations.take();
        ops.push(operand);
        self.operations.set(ops);

    }

    //same here
    fn add_value(self, value: Types) -> Result<Types, ()>{
        let mut vals = self.values.take();
        vals.push(value);
        self.values.set(vals);

        Ok(Inoperable(self))
    }
}

impl Fraction {
    fn to_float(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }


    fn simplify(self) -> Fraction {
        let gcd = gcd(self.numerator, self.denominator);

        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd
        }
    }
}

//using Euclidean algorithm
fn gcd(num1: i64, num2: i64) -> i64{
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
    #[test]
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

        assert_eq!(var3.coefficient, 5.0);
    }

    #[test]
    fn adding_variables_different_power() {
        let var1 = Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        };
        let var2 = Variable {
            symbol: 'y',
            power: 2.0,
            coefficient: 4.0,
        };

        assert_eq!(Err(()), Variable::add(var1, Variable(var2)));
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
