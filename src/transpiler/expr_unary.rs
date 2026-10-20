use full_moon::ast::{Expression, UnOp};

use crate::transpiler::{Transpile, minus_token, stmt::TranspiledExpr};

impl Transpile for syn::ExprUnary {
    type Output = TranspiledExpr;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        match self.op {
            syn::UnOp::Deref(star) => todo!(),
            syn::UnOp::Not(not) => todo!(),
            syn::UnOp::Neg(_) => Ok(TranspiledExpr::expr_only(Expression::UnaryOperator {
                unop: UnOp::Minus(minus_token()),
                expression: Box::new(self.expr.transpile(ctx)?.value),
            })),
            _ => todo!(),
        }
    }
}
