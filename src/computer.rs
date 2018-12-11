//! For taking the product of the parser and calculating it into a 
//! a final form. In this case, the final form is an f64.

use crate::lexer::*;
use crate::parser::*;

// If you come bearing big changes, you may have to rewrite
// this to suit your needs.

/// Turn an AST / Expr into an f64.
pub fn compute(expr: &Expr) -> f64 {
    match expr {
        Expr::Constant(num) => *num,
        Expr::Identifier(_) => 0.,
        Expr::Neg(expr) => -compute(expr),
        Expr::BinOp(op, lexpr, rexpr) => {
            let lnum = compute(&lexpr);
            let rnum = compute(&rexpr);

            match op {
                Operator::Plus => lnum + rnum,
                Operator::Minus => lnum - rnum,
                Operator::Star => lnum * rnum,
                Operator::Slash => lnum / rnum,
                Operator::Percent => lnum % rnum,
                _ => unimplemented!(),
            }
        }
        Expr::Function(function, expr) => {
            let num = compute(&expr);
            match function {
                Function::Sqrt => num.sqrt(),
                Function::Sin => num.sin(),
                Function::Cos => num.cos(),
                Function::Tan => num.tan(),
                Function::Log => num.log10(),
                Function::Abs => num.abs(),
            }
        }
        Expr::Pow(lexpr, rexpr) => {
            compute(&lexpr).powf(compute(&rexpr))
        }
    }
}
