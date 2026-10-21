use full_moon::ast::{Expression, UnOp};
use full_moon::tokenizer::Symbol;

use crate::mappings::expr::{BoundExpr, IntermediateExpr};
use crate::{Transpile, symbol};
use crate::{TranspileError, TranspileOptions, UniversalTranspileCtx};

impl Transpile for syn::ExprUnary {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        let intermediate_expr = self.expr.transpile_default_ctx(u_ctx, opts)?;
        let un_op = match &self.op {
            syn::UnOp::Deref(star) => todo!(),
            syn::UnOp::Not(_) => UnOp::Not(symbol(Symbol::Not)),
            syn::UnOp::Neg(_) => UnOp::Minus(symbol(Symbol::Minus)),
            _ => todo!(),
        };
        Ok(match intermediate_expr {
            IntermediateExpr::Simple(expression) => {
                IntermediateExpr::Simple(apply_unary(un_op, expression))
            }
            IntermediateExpr::Bound(BoundExpr {
                leading_stmts,
                expr,
            }) => IntermediateExpr::bound(leading_stmts, apply_unary(un_op, expr)),
        })
    }
}

pub fn apply_unary(un_op: UnOp, expr: Expression) -> Expression {
    Expression::UnaryOperator {
        unop: un_op,
        expression: Box::new(expr),
    }
}
