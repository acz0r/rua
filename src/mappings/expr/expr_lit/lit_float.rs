use crate::{
    Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx,
    mappings::expr::IntermediateExpr, num_lit,
};
impl Transpile for syn::LitFloat {
    type Ctx = ();
    type Output = crate::mappings::expr::IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut UniversalTranspileCtx,
        _opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(IntermediateExpr::Simple(num_lit(self.base10_digits())))
    }
}
