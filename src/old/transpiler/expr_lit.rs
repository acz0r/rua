use crate::transpiler::{Transpile, TranspileWithoutCtx, expr::IntermediateExpr};

impl Transpile for syn::ExprLit {
    type Output = IntermediateExpr;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        match &self.lit {
            syn::Lit::Str(lit_str) => Ok(IntermediateExpr::expr_only(lit_str.transpile()?)),
            // syn::Lit::ByteStr(lit_byte_str) => todo!(),
            // syn::Lit::CStr(lit_cstr) => todo!(),
            // syn::Lit::Byte(lit_byte) => todo!(),
            syn::Lit::Char(lit_char) => Ok(IntermediateExpr::expr_only(lit_char.transpile()?)),
            syn::Lit::Int(lit_int) => Ok(IntermediateExpr::expr_only(lit_int.transpile()?)),
            syn::Lit::Float(lit_float) => Ok(IntermediateExpr::expr_only(lit_float.transpile()?)),
            syn::Lit::Bool(lit_bool) => Ok(IntermediateExpr::expr_only(lit_bool.transpile()?)),
            // syn::Lit::Verbatim(literal) => todo!(),
            _ => unimplemented!("Unsupported literal type"),
        }
    }
}
