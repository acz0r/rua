use full_moon::{
    ShortString,
    tokenizer::{Token, TokenReference, TokenType},
};

use crate::transpiler::{Transpile, token_ref};

impl Transpile for syn::FnArg {
    type Output = TokenReference;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        let ident = match self {
            syn::FnArg::Receiver(_) => unimplemented!("`self` arg not supported"),
            syn::FnArg::Typed(pat_type) => pat_type.pat.transpile(ctx)?,
        };

        Ok(token_ref(Token::new(TokenType::Identifier {
            identifier: ShortString::new(ident.to_string()),
        })))
    }
}
