mod numbers;

#[allow(unused_imports)]
use crate::evaluator::numbers::{
    Types::{self, *},
    Operand::{self, *},
    Variable,
    Fraction,
    Operations, // Operations trait
    Expression,
};

#[allow(dead_code)]
pub enum ExpressionComponents {
    Type(Types),
    Op(Operand),
}

use ExpressionComponents::*;

#[allow(dead_code)]
pub fn create_stack(_commands: &str) -> Vec<ExpressionComponents> {

    //create a vector containing enum variants for types and operations
    //evaluate the vector in the evaluate_stack function by separating
    //it into operations and types (which the Expression enum is useful for)

    let val1 = Type(Float(11.4));
    let val2 = Type(Float(19.3));
    let op = Op(Multiply);

    let eval_vec: Vec<ExpressionComponents> = vec![val1, op, val2];
    eval_vec

}

#[allow(dead_code)]
pub fn evaluate_stack(stack: &mut Vec<ExpressionComponents>) -> Vec<ExpressionComponents> {
    let mut ops: Vec<Operand> = Vec::new();
    let mut nums: Vec<Types> = Vec::new();
    let mut inoperable_expression: Vec<ExpressionComponents> = Vec::new();

    while !stack.is_empty() {
        let curr_val = stack.pop().unwrap();

        match curr_val {
            Type(t) => nums.push(t),
            Op(LeftParenthesis) => ops.push(LeftParenthesis),
            Op(RightParenthesis) => {
                while ops.pop() != Some(LeftParenthesis) {
                    pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
                }
                stack.pop(); //get rid of Left paranthesis
            },

            Op(operator) => {
                while !ops.is_empty() && ops[0].priority() >= operator.priority() {
                    pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
                }

                ops.push(operator);
            },
        }

        while !ops.is_empty() {
            pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
        }
    }

    // push the simplified value to inoperable expression
    inoperable_expression.push(Type(nums.pop().unwrap()));

    inoperable_expression
 }

// simply abstracted this behaviour to a function since its called multiple times above
 fn pop_expression(nums: &mut Vec<Types>, ops: &mut Vec<Operand>, inoperable_expression: &mut Vec<ExpressionComponents>) {
     let exp = Expression {
         values: vec![nums.pop().unwrap(), nums.pop().unwrap()],
         operation: ops.pop().unwrap(),
     };

     match &exp.values[0] {
         Expression(exp) => {
             // split the Expression, and push to inoperable_expression
            { // brackets here for scoping reasons
                inoperable_expression.push(Type(exp.values[0].clone()));
                inoperable_expression.push(Op(exp.operation.clone()));
                inoperable_expression.push(Type(exp.values[1].clone()));
            }
             //other setup
             inoperable_expression.push(Op(exp.operation.clone()));
             let value = exp.values[1].clone();
             let operation = ops.pop().unwrap();

             // basically clears information from nums and ops until we have something we can actually work with
             while operation != Add && operation != Subtract {
                 // push stuff onto inoperable so that we can ignore it
                 inoperable_expression.push(Type(value.clone()));
                 inoperable_expression.push(Op(operation.clone()));

                 // redefine these values, so that we can do it again
                 if nums.len() >= 2 {
                     #[allow(unused_variables)]
                     let value = nums.pop().unwrap();
                     #[allow(unused_variables)]
                     let operation = ops.pop().unwrap();
                 } else { // if we run out of nums, we need to return
                    if nums.len() == 1 {
                        inoperable_expression.push(Type(nums.pop().unwrap())); //adds the final num
                        inoperable_expression.push(Op(ops.pop().unwrap()));
                    }
                    return ();
                 }

             } // then loop again

            // this effectively represents a "breakpoint", a spot where we can stop pushing to the inoperable nums

            // put the final type and operation on the inoperablenums_stack
            inoperable_expression.push(Type(value.clone()));
            inoperable_expression.push(Op(operation.clone()));

            // then stop this method
            return ();
         }
        _ => (),// do nothing
     };

     match exp.values[1] {
         Expression(_) => { //put back on stack
             // im pretty positive this is actually an error case,
             panic!("Why was an expression at exp.values.1? mod.rs line 127");
         }
        _ => (),// do nothing
     };

     let returned = evaluate_expression(exp).unwrap_or_else(|_e| {
         eprintln!("Failed");
         std::process::exit(1);
     });

     nums.push(returned);
 }

 //this function will translate an operator into an expression evaluation
 //using the functions we made in numbers.rs
pub fn evaluate_expression(expression: Expression) -> Result<Types, ()> {
    /*  youll notice weird redundancy going on here
        its because each one of the types inside an enum is different,
        so each case must be written out explicitly :(
    */
    match expression.values[0].clone() {
        Float(t) => get_operation((t, expression.values[1].clone()), expression.operation),
        Fraction(t) => get_operation((t, expression.values[1].clone()), expression.operation),
        Variable(t) => get_operation((t, expression.values[1].clone()), expression.operation),
        _ => Err(()),
    }
}

fn get_operation<T: Operations>(values: (T, Types), op: Operand) -> Result<Types, ()> {
    match op {
        // Exponent => Operations::exponentiate(values.0, values.1),
        Multiply => Operations::multiply(values.0, values.1),
        Divide => Operations::divide(values.0, values.1),
        Subtract => Operations::sub(values.0, values.1),
        Add => Operations::add(values.0, values.1),
        _ => Err(()),
    }
}
