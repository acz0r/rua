use crate::{Transpile, mappings::expr::IntermediateExpr, num_lit};

impl Transpile for syn::LitInt {
    type Ctx = ();
    type Output = crate::mappings::expr::IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut crate::UniversalTranspileCtx,
        _opts: &crate::TranspileOptions,
    ) -> Result<Self::Output, crate::TranspileError> {
        Ok(IntermediateExpr::Simple(num_lit(self.base10_digits())))
    }
}
