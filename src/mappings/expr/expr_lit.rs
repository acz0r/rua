use crate::Transpile;
use crate::mappings::expr::IntermediateExpr;
use crate::{TranspileError, TranspileOptions, UniversalTranspileCtx};

pub mod lit_bool;
pub mod lit_char;
pub mod lit_float;
pub mod lit_int;
pub mod lit_str;

impl Transpile for syn::ExprLit {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(match &self.lit {
            syn::Lit::Str(lit_str) => lit_str.transpile_default_ctx(u_ctx, opts)?,
            syn::Lit::ByteStr(lit_byte_str) => todo!(),
            syn::Lit::CStr(lit_cstr) => todo!(),
            syn::Lit::Byte(lit_byte) => todo!(),
            syn::Lit::Char(lit_char) => lit_char.transpile_default_ctx(u_ctx, opts)?,
            syn::Lit::Int(lit_int) => lit_int.transpile_default_ctx(u_ctx, opts)?,
            syn::Lit::Float(lit_float) => lit_float.transpile_default_ctx(u_ctx, opts)?,
            syn::Lit::Bool(lit_bool) => lit_bool.transpile_default_ctx(u_ctx, opts)?,
            syn::Lit::Verbatim(literal) => todo!(),
            _ => todo!(),
        })
    }
}
