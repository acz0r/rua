use crate::transpiler::{Transpile, stmt::TranspiledExpr};

impl Transpile for syn::ExprLit {
    type Output = TranspiledExpr;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        match &self.lit {
            syn::Lit::Str(lit_str) => Ok(TranspiledExpr::expr_only(lit_str.transpile(ctx)?)),
            // syn::Lit::ByteStr(lit_byte_str) => todo!(),
            // syn::Lit::CStr(lit_cstr) => todo!(),
            // syn::Lit::Byte(lit_byte) => todo!(),
            syn::Lit::Char(lit_char) => Ok(TranspiledExpr::expr_only(lit_char.transpile(ctx)?)),
            syn::Lit::Int(lit_int) => Ok(TranspiledExpr::expr_only(lit_int.transpile(ctx)?)),
            syn::Lit::Float(lit_float) => Ok(TranspiledExpr::expr_only(lit_float.transpile(ctx)?)),
            syn::Lit::Bool(lit_bool) => Ok(TranspiledExpr::expr_only(lit_bool.transpile(ctx)?)),
            // syn::Lit::Verbatim(literal) => todo!(),
            _ => unimplemented!("Unsupported literal type"),
        }
    }
}
