#[allow(unused_imports)]

pub mod operations;
pub mod numbers;

use self::numbers::*;
use self::operations::*;

pub fn evaluate_stack(stack: &mut Vec<ExpressionComponents>) -> Vec<ExpressionComponents> {
    let mut ops: Vec<Operand> = Vec::new();
    let mut nums: Vec<Types> = Vec::new();
    let mut inoperable_expression: Vec<ExpressionComponents> = Vec::new();

    while !stack.is_empty() {
        println!("input vec: {:?}", stack);

        let curr_val = stack.pop().unwrap();

        match curr_val {
            Type(t) => nums.push(t),
            Op(LeftParenthesis) => {
                while ops.last().unwrap().clone() != RightParenthesis {
                    pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
                }
                ops.pop(); //get rid of Left paranthesis
            },
            Op(RightParenthesis) => ops.push(RightParenthesis),
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

        println!("inoperable vec: {:?}", inoperable_expression);
        println!("\nnums vec: {:?}\n", nums);

    }

    println!("\nnums vec 49: {:?}\n", nums);


    while !ops.is_empty() {
	//println!("\n----popping ops");
        pop_expression(&mut nums, &mut ops, &mut inoperable_expression);
    }

    println!("\nnums vec 57: {:?}\n", nums);


    // push the simplified value to inoperable expression
    // or turn a final expression into its components and push to inoperable_expression
    // don't unwrap right away since our nums vector might be empty
    match nums.pop() {
	Some(inoperable) => {
	    match inoperable {
		Expression(exp) => {
		    inoperable_expression.push(Type(exp.values[0].clone()));
		    inoperable_expression.push(Op(exp.operation.clone()));
		    inoperable_expression.push(Type(exp.values[1].clone()));
		},
		other => inoperable_expression.push(Type(other)),
	    }
	},
	None => {}
    }

    inoperable_expression
}

// simply abstracted this behaviour to a function since its called multiple times above
 fn pop_expression(nums: &mut Vec<Types>, ops: &mut Vec<Operand>, inoperable_expression: &mut Vec<ExpressionComponents>) {
//     println!("\nOps Stack: {:?}", ops);
//     println!("Types Stack: {:?}", nums);
    // println!("\nNums: {:?},\nOps: {:?}\n", &nums, &ops);
     let mut exp = Expression {
         values: vec![nums.pop().unwrap(), nums.pop().unwrap()],
         operation: ops.pop().unwrap(),
     };

     match &exp.values[1] {
         Expression(_) => {
             exp.values.swap(0,1);
         },
         _ => {},
     }

     match &exp.values[0] {
         Expression(internal_exp) => {
             let returned = evaluate_expression(exp.clone());

             println!("\n{:?} != {:?}?\n", returned, Expression(exp.clone()));

             if returned != Expression(exp.clone()) {
                 // try and evaluate remaining expression
                 let unwrapped_returned = match returned.clone() {
                     Expression(t) => t,
                     _ => panic!(),
                 };
                 let internal_returned = evaluate_expression(unwrapped_returned);

                 if returned != internal_returned {
                     println!("extra swag.");
                     nums.push(internal_returned);
                     return ();
                 }

                 println!("swag");
                 nums.push(returned);
                 return ();
             }

             // split the Expression, and push to inoperable_expression
            { // brackets here for scoping reasons
                inoperable_expression.push(Type(internal_exp.values[0].clone()));
                inoperable_expression.push(Op(internal_exp.operation.clone()));
                inoperable_expression.push(Type(internal_exp.values[1].clone()));
            }
             //other setup
             inoperable_expression.push(Op(exp.operation.clone()));

             if ops.len() < 1 {
                 inoperable_expression.push(Type(exp.values[1].clone()));
                 return ();
             }

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

     /*match exp.values[1] {
         Expression(_) => { //put back on stack
             // im pretty positive this is actually an error case, ~Logan
             panic!("Why was an expression at exp.values.1? mod.rs line 127");
         }
        _ => (),// do nothing
    };*/

     let returned = evaluate_expression(exp);

     nums.push(returned);
 }

 //this function will translate an operator into an expression evaluation
 //using the functions we made in numbers.rs 🤡🤡🤡
pub fn evaluate_expression(expression: Expression) -> Types {
    /*  youll notice weird redundancy going on here
        its because each one of the types inside an enum is different,
        so each case must be written out explicitly :(
    */

    let returned = match expression.values[0].clone() {
        Float(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        Fraction(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        Variable(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
        Expression(t) => get_operation((t, expression.values[1].clone()), expression.operation.clone()),
    };

    match returned { // iff the operation returned error, return the input expression
        Ok(t) => t,
        Err(_) => Expression(expression),
    }
}

fn get_operation<T: Operations>(values: (T, Types), op: Operand) -> Result<Types, ()> {
    match op { // 🤡🤡
        // Exponent => Operations::exponentiate(values.0, values.1),
        Multiply => Operations::multiply(values.0, values.1),
        Divide => Operations::divide(values.0, values.1),
        Subtract => Operations::sub(values.0, values.1),
        Add => Operations::add(values.0, values.1),
        Exponent => Operations::exponentiate(values.0, values.1),
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

    #[test]
    fn float_order_of_operations(){
        let mut stack = vec![
        Op(LeftParenthesis),
        Type(Float(1.0)), Op(Add), Type(Float(5.0)),
        Op(RightParenthesis),
        Op(Multiply), Type(Float(3.0))];

        let answer = evaluate_stack(&mut stack);
        println!("answer: {:?}", &answer);

        assert_eq!(vec![Type(Float(18.0))], answer);
    }

    #[test]
    fn variable_add_float(){
        let stack = vec![
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
    fn variable_division_order_of_operations(){
        let mut stack = vec![Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 9.0,
        })),
        Op(Divide),
        Op(LeftParenthesis),
        Type(Float(3.0)), Op(Multiply), Type(Float(2.0)), Op(Subtract), Type(Float(3.0)),
        Op(RightParenthesis)];

        let cmp_answer = vec![Type(Variable(Variable{
            symbol: 'x',
            power: 1.0,
            coefficient: 3.0,
        }))];

        let answer = evaluate_stack(&mut stack);

        assert_eq!(answer, cmp_answer);

    }

    #[test]
    fn divide_float_by_variable(){
        let mut stack = vec![Type(Float(2.5)),
        Op(Divide),
        Type(Variable(Variable{
            symbol: 'z',
            power: 1.0,
            coefficient: 5.0
        }))];

        let cmp_answer = vec![Type(Variable(Variable{
            symbol: 'z',
            power: -1.0,
            coefficient: 0.5
        }))];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));
    }

    #[test]
    fn exponentiate_floats() {

        let mut stack = vec![Type(Float(3.0)), Op(Exponent), Type(Float(3.0)),
        Op(Add), Type(Float(9.0)), Op(Exponent), Type(Variable(Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        }))];

        let cmp_answer = vec![Type(Float(27.0)), Op(Add),
        Type(Expression(Expression {
            values: vec![Float(9.0), Variable(Variable {
                symbol: 'x',
                power: 1.0,
                coefficient: 1.0 })],
            operation: Exponent })
        )];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));

    }

    #[test] fn exponentiate_variables() {

        let mut stack = vec![Type(Variable(Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.0,
        })), Op(Exponent), Type(Float(3.0))];


        let cmp_answer = vec![Type(Variable(Variable {
            symbol: 'x',
            power: 3.0,
            coefficient: 1.0,
        }))];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));

    }

    #[test]
    fn exponentiate_fraction() {
        let mut stack = vec![Type(Fraction(Fraction {
            numerator: 2,
            denominator: 3,
        })),
        Op(Exponent), Type(Float(3.0))];

        let cmp_answer = vec![Type(Fraction(Fraction {
            numerator: 8,
            denominator: 27,
        }))];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));

    }

    #[test]
    fn fractional_exponents() {

        let mut stack = vec![Type(Fraction(Fraction {
            numerator: 169,
            denominator: 144,
        })), Op(Exponent),
        Type(Fraction(Fraction {
            numerator: 1,
            denominator: 2,
        }))];

        let cmp_answer = vec![Type(Fraction(Fraction {
            numerator: 13,
            denominator: 12,
        }))];

        assert_eq![cmp_answer, evaluate_stack(&mut stack)];
    }

    #[test]
    fn dividing_vars() {
        let mut stack = vec![
        Type(Float(2.0)),
        Op(Multiply),
        Type(Variable(Variable {
            symbol: 'x',
            power: 3.0,
            coefficient: 2.0,
        })),
        Op(Divide),
        Type(Variable(Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 1.8,
        })),
        ];


        let cmp_answer = vec![Type(Variable(Variable {
            symbol: 'x',
            power: 2.0,
            coefficient: 4.0/1.8,
        }))];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));
    }

    #[test]
    fn multiplying_coefficients() {
        let mut stack = vec![
        Type(Float(2.0)),
        Op(Multiply),
        Type(Variable(Variable {
            symbol: 'x',
            power: 1.0,
            coefficient: 2.0,
        })),];

        let cmp_answer = vec![Type(Variable(Variable {
            symbol:'x',
            power: 1.0,
            coefficient: 4.0
        }))];

        assert_eq!(cmp_answer, evaluate_stack(&mut stack));
    }

}
