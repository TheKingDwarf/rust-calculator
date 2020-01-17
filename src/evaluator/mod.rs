mod numbers;

use crate::evaluator::numbers::{
    Types::{self, *},
    Operand::{self, *},
    Variable,
    Fraction,
    Operations, // Operations trait
};

pub enum Expression {
    Type(Types),
    Op(Operand),
}
use Expression::*;

pub fn create_stack(commands: &str) -> Vec<Expression> {
    //create a vector containing enum variants for types and operations
    //evaluate the vector in the evaluate_stack function by separating
    //it into operations and types (which the Expression enum is useful for)

    let val1 = Type(Float(11.4));
    let val2 = Type(Float(19.3));
    let op = Op(Multiply);

    let eval_vec: Vec<Expression> = vec![val1, op, val2];
    eval_vec

}

pub fn evaluate_stack(stack: &mut Vec<Expression>) {
    let mut ops: Vec<Operand> = Vec::new();
    let mut nums: Vec<Types> = Vec::new();

    while !stack.is_empty() {
        let curr_val = stack.pop().unwrap();

        match curr_val {
            Type(t) => nums.push(t),
            Op(LeftParenthesis) => ops.push(LeftParenthesis),
            Op(RightParenthesis) => {
                while ops.pop() != Some(LeftParenthesis) {
                    pop_expression(&mut nums, &mut ops);
                }
                stack.pop(); //get rid of Left parenthesis
            },

            Op(operator) => {
                while !ops.is_empty() && ops[0].priority() >= operator.priority() {
                    pop_expression(&mut nums, &mut ops);
                }

                ops.push(operator);
            },

            _ => println!(""),
        }

        while !ops.is_empty() {
            pop_expression(&mut nums, &mut ops);
        }
    }
 }

// simply abstracted this behaviour to a function since its called multiple times above
 fn pop_expression(nums: &mut Vec<Types>, ops: &mut Vec<Operand>) {
     let values = (nums.pop().unwrap(), nums.pop().unwrap());

     let operation = ops.pop().unwrap();

     let returned = evaluate_expression(values, operation).unwrap_or_else(|e| {
         eprintln!("Failed");
         std::process::exit(1);
     });

     nums.push(returned);
 }

 //this function will translate an operator into an expression evaluation
 //using the functions we made in numbers.rs
pub fn evaluate_expression(values: (Types, Types), op: Operand) -> Result<Types, ()> {
    /*  youll notice weird redundancy going on here
        its because each one of the types inside an enum is different,
        so each case must be written out explicitly :(
    */
    match values.0 {
        Float(float) => get_operation((float, values.1), op),
        Fraction(fraction) => get_operation((fraction, values.1), op),
        Variable(variable) => get_operation((variable, values.1), op),
        Inoperable(inoperable) => get_operation((inoperable, values.1), op),
    }
}

fn get_operation<T: Operations>(values: (T, Types), op: Operand) -> Result<Types, ()> {
    match op {
        // Exponent => Operations::exponentiate(values.0, values.1),
        Multiply => Operations::multiply(values.0, values.1),
        Divide => Operations::divide(values.0, values.1),
        Subtract => Operations::sub(values.0, values.1),
        Add => Operations::add(values.0, values.1),
        //because we have parenthesis, etc as part of our Operand enum
        //we must cover the rest of the cases
        _ => Err(()),
    }
}
