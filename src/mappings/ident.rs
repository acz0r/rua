use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx, ident};
use full_moon::tokenizer::TokenReference;

impl Transpile for syn::Ident {
    type Ctx = ();
    type Output = TokenReference;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut UniversalTranspileCtx,
        _opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        // Transpile ident
        Ok(ident(&self.to_string()))
    }
}
