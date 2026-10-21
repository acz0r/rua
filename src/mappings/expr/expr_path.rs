use full_moon::ast::{Expression, Var};

use crate::{
    Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx,
    mappings::expr::IntermediateExpr,
};
impl Transpile for syn::ExprPath {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        // TODO: support more complex paths
        let path = self
            .path
            .get_ident()
            .unwrap()
            .transpile_default_ctx(u_ctx, opts)?;

        Ok(IntermediateExpr::Simple(Expression::Var(Var::Name(path))))
    }
}
