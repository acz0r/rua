use crate::transpiler::{Transpile, TranspileCtx, TranspileError, stmt::TranspiledExpr};

impl Transpile for syn::ExprIf {
    type Output = TranspiledExpr;

    fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
        let cond = self.cond.transpile(ctx)?;
        let then_branch = self.then_branch.transpile(ctx)?;
        let else_branch = if let Some((_, else_expr)) = &self.else_branch {
            Some(else_expr.transpile(ctx)?)
        } else {
            None
        };

        Ok(TranspiledExpr::)
    }
}
