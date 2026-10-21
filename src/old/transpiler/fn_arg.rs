use full_moon::{
    ShortString,
    tokenizer::{Token, TokenReference, TokenType},
};

use crate::transpiler::{Transpile, TranspileWithoutCtx, token_ref};

impl Transpile for syn::FnArg {
    type Output = TokenReference;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        let ident = match self {
            syn::FnArg::Receiver(_) => unimplemented!("`self` arg not supported"),
            syn::FnArg::Typed(pat_type) => pat_type.pat.transpile()?,
        };

        Ok(token_ref(Token::new(TokenType::Identifier {
            identifier: ShortString::new(ident.to_string()),
        })))
    }
}
