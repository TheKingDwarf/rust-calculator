#[allow(unused_imports)]
use crate::operations::*;


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
                while ops.last().unwrap().clone() != LeftParenthesis {
                    pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
                }
                ops.pop(); //get rid of Left paranthesis
            },

            Op(operator) => {
                if !ops.is_empty() {
                    while !ops.is_empty() {
                        let top = ops.last().unwrap().clone(); //non-destructively checks the top member of ops

                        if top.priority() < operator.priority() || top.is_parenthesis() {
                            break;
                        }

                        pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
                    }
                }

                ops.push(operator);
            },
        }

    }

    while !ops.is_empty() {
        pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
    }

    // push the simplified value to inoperable expression
    // or turn a final expression into its componenets and push to inoperable_expression
    match nums.pop().unwrap() {
        Expression(exp) => {
            inoperable_expression.push(Type(exp.values[0].clone()));
            inoperable_expression.push(Op(exp.operation.clone()));
            inoperable_expression.push(Type(exp.values[1].clone()));
        },
        other => inoperable_expression.push(Type(other)),
    }


    inoperable_expression
 }

// simply abstracted this behaviour to a function since its called multiple times above
 fn pop_expression(nums: &mut Vec<Types>, ops: &mut Vec<Operand>, inoperable_expression: &mut Vec<ExpressionComponents>) {
     //println!("Nums: {:?}, Ops: {:?}", &nums, &ops);
     let exp = Expression {
         values: vec![nums.pop().unwrap(), nums.pop().unwrap()],
         operation: ops.pop().unwrap(),
     };

     match &exp.values[0] {
         Expression(internal_exp) => {
             // split the Expression, and push to inoperable_expression
            { // brackets here for scoping reasons
                inoperable_expression.push(Type(internal_exp.values[0].clone()));
                inoperable_expression.push(Op(internal_exp.operation.clone()));
                inoperable_expression.push(Type(internal_exp.values[1].clone()));
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
             // im pretty positive this is actually an error case, ~Logan
             panic!("Why was an expression at exp.values.1? mod.rs line 127");
         }
        _ => (),// do nothing
     };

     let returned = evaluate_expression(exp);

     nums.push(returned);
 }

 //this function will translate an operator into an expression evaluation
 //using the functions we made in numbers.rs
pub fn evaluate_expression(expression: Expression) -> Types {
    /*  youll notice weird redundancy going on here
        its because each one of the types inside an enum is different,
        so each case must be written out explicitly :(
    */
    let returned = match expression.values[0].clone() {
        Float(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        Fraction(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        Variable(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        _ => Err(()),
    };

    match returned { // iff the operation returned error, return the input expression
        Ok(t) => t,
        Err(_) => Expression(expression),
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


#[cfg(test)]
mod tests {
    use super::*;
    //pub use numbers::Fraction;

    #[test]
    fn multiplying_floats(){
        let mut stack = vec![Type(Float(11.4)), Op(Multiply), Type(Float(19.3))];

        assert_eq!(vec![Type(Float(220.02))], evaluate_stack(&mut stack));
    }

    #[test]
    fn add_fraction_to_float(){
        let mut stack = vec![Type(Float(11.25)), Op(Add), Type(Fraction(
            Fraction{
                numerator: 3,
                denominator: 4
            }))];

        assert_eq!(vec![Type(Float(12.0))], evaluate_stack(&mut stack));
    }

    #[test]
    fn floats_to_fraction() { //check that floats can be converted to fractions
        let mut stack = vec![Type(Float(3.0)), Op(Divide), Type(Float(4.0))];

        assert_eq!(vec![Type(Fraction(Fraction {
            numerator: 3,
            denominator: 4
        }))], evaluate_stack(&mut stack));
    }

    fn floats_to_float_not_fraction() { //check that we dont convert floats to fractions unnescarilly
        let mut stack = vec![Type(Float(8.0)), Op(Divide), Type(Float(4.0))];

        assert_ne!(vec![Type(Fraction(Fraction {
            numerator: 8,
            denominator: 4
        }))], evaluate_stack(&mut stack));
    }

    //OOO = order of operations
    #[test]
    fn float_OOO(){
        let mut stack = vec![
        Op(LeftParenthesis),
        Type(Float(1.0)), Op(Add), Type(Float(5.0)),
        Op(RightParenthesis),
        Op(Multiply), Type(Float(3.0))];

        stack.reverse();

        let answer = evaluate_stack(&mut stack);
        println!("answer: {:?}", &answer);

        assert_eq!(vec![Type(Float(18.0))], answer);
    }

    #[test]
    fn variable_add_float(){
        let mut stack = vec![
            Type(Variable(Variable {
                symbol: 'x',
                power: 1.0,
                coefficient: 1.0,
            })),
            Op(Add),
            Type(Float(4.0))
        ];

        let answer = evaluate_stack(&mut stack.clone());
        println!("answer: {:?}", &answer);

        assert_eq!(stack, answer);
    }

    #[test]
    fn variable_div_by_float(){
        let mut stack = vec![Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 9.0,
        })), Op(Divide), Type(Float(3.0))];

        let answer = evaluate_stack(&mut stack);

        let cmp_answer = Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 3.0,
        }));

        assert_eq!(vec![cmp_answer], answer);


    }

    #[test]
    fn dividing_floats(){
        let mut stack = vec![Type(Float(220.02)), Op(Divide), Type(Float(11.4))];

        let answer = evaluate_stack(&mut stack);

        assert_eq!(answer, vec![Type(Float(19.3))]);

    }

    #[test]
    fn variable_division_OOO(){
        let mut stack = vec![Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 9.0,
        })),
        Op(Divide), Op(LeftParenthesis),
        Type(Float(3.0)), Op(Multiply), Type(Float(2.0)), Op(Add), Type(Float(3.0)),
        Op(RightParenthesis)];

        let cmp_answer = vec![Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        }))];

        let answer = evaluate_stack(&mut stack);

        assert_eq!(answer, cmp_answer);

    }

}
