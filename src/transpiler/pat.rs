use full_moon::{ast::Expression, tokenizer::TokenReference};

use crate::transpiler::Transpile;

impl Transpile for syn::Pat {
    type Output = TokenReference;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        match self {
            // syn::Pat::Const(expr_const) => todo!(),
            // syn::Pat::Guard(pat_guard) => todo!(),
            syn::Pat::Ident(pat_ident) => Ok(pat_ident.ident.transpile(ctx)?),
            // syn::Pat::Lit(expr_lit) => todo!(),
            // syn::Pat::Macro(expr_macro) => todo!(),
            // syn::Pat::Or(pat_or) => todo!(),
            // syn::Pat::Paren(pat_paren) => todo!(),
            // syn::Pat::Path(expr_path) => todo!(),
            // syn::Pat::Range(expr_range) => todo!(),
            // syn::Pat::Reference(pat_reference) => todo!(),
            // syn::Pat::Rest(pat_rest) => todo!(),
            // syn::Pat::Slice(pat_slice) => todo!(),
            // syn::Pat::Struct(pat_struct) => todo!(),
            // syn::Pat::Tuple(pat_tuple) => todo!(),
            // syn::Pat::TupleStruct(pat_tuple_struct) => todo!(),
            // syn::Pat::Type(pat_type) => todo!(),
            // syn::Pat::Verbatim(token_stream) => todo!(),
            // syn::Pat::Wild(pat_wild) => todo!(),
            _ => unimplemented!("Unsupported pattern"),
        }
    }
}
