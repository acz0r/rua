use full_moon::ast::{BinOp, Expression, LocalAssignment, Stmt, Var};
use full_moon::tokenizer::{Symbol, TokenReference};

use crate::mappings::expr::IntermediateExpr;
use crate::{Transpile, single, symbol, unsupported};
use crate::{TranspileError, TranspileOptions, UniversalTranspileCtx};

impl Transpile for syn::ExprBinary {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        let bin_op = match &self.op {
            syn::BinOp::Add(_) => BinOp::Plus(symbol(Symbol::Plus)),
            syn::BinOp::Sub(_) => BinOp::Minus(symbol(Symbol::Minus)),
            syn::BinOp::Mul(_) => BinOp::Star(symbol(Symbol::Star)),
            syn::BinOp::Div(_) => BinOp::Slash(symbol(Symbol::Slash)),

            syn::BinOp::Eq(_) => BinOp::TwoEqual(symbol(Symbol::TwoEqual)),
            syn::BinOp::Ne(_) => BinOp::TildeEqual(symbol(Symbol::TildeEqual)),
            syn::BinOp::Lt(_) => BinOp::LessThan(symbol(Symbol::LessThan)),
            syn::BinOp::Le(_) => BinOp::LessThanEqual(symbol(Symbol::LessThanEqual)),
            syn::BinOp::Gt(_) => BinOp::GreaterThan(symbol(Symbol::GreaterThan)),
            syn::BinOp::Ge(_) => BinOp::GreaterThanEqual(symbol(Symbol::GreaterThanEqual)),

            _ => unsupported!("binary operator", "operator lowering is not implemented"),
        };

        let left = self.left.transpile_default_ctx(u_ctx, opts)?;
        let right = self.right.transpile_default_ctx(u_ctx, opts)?;

        // Proposed context method; you still need to implement this allocator.
        let left_temp = u_ctx.unique_var();

        Ok(combine_eager_binary(left, right, bin_op, left_temp))
    }
}

pub fn combine_eager_binary(
    left: IntermediateExpr,
    right: IntermediateExpr,
    bin_op: BinOp,
    left_temp: TokenReference,
) -> IntermediateExpr {
    let (mut leading_stmts, left_expr) = left.split();
    let (right_stmts, right_expr) = right.split();

    // Evaluate the left operand before anything belonging to the right.
    leading_stmts.push(Stmt::LocalAssignment(
        LocalAssignment::new(single(left_temp.clone()))
            .with_equal_token(Some(symbol(Symbol::Equal)))
            .with_expressions(single(left_expr)),
    ));

    leading_stmts.extend(right_stmts);

    IntermediateExpr::bound(
        leading_stmts,
        Expression::BinaryOperator {
            binop: bin_op,
            lhs: Box::new(Expression::Var(Var::Name(left_temp))),
            rhs: Box::new(right_expr),
        },
    )
}
