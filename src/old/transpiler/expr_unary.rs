use full_moon::ast::{Expression, UnOp};

use crate::transpiler::{Transpile, TranspileWithoutCtx, expr::IntermediateExpr, minus_token};

impl Transpile for syn::ExprUnary {
    type Output = IntermediateExpr;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        match self.op {
            syn::UnOp::Deref(star) => todo!(),
            syn::UnOp::Not(not) => todo!(),
            syn::UnOp::Neg(_) => Ok(IntermediateExpr::expr_only(Expression::UnaryOperator {
                unop: UnOp::Minus(minus_token()),
                expression: Box::new(self.expr.transpile()?.value_expr),
            })),
            _ => todo!(),
        }
    }
}
