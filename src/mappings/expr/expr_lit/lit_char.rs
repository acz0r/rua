use crate::{
    Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx,
    mappings::expr::{IntermediateExpr, expr_lit::lit_str::escape},
    str_lit,
};

impl Transpile for syn::LitChar {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut UniversalTranspileCtx,
        _opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(IntermediateExpr::Simple(str_lit(escape(
            self.value().to_string(),
        ))))
    }
}
