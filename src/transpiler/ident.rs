use crate::transpiler::{Transpile, TranspileError, token_ref};
use full_moon::{
    ShortString,
    tokenizer::{Token, TokenReference, TokenType},
};

impl Transpile for syn::Ident {
    type Output = TokenReference;

    fn transpile(&self, _ctx: &mut super::TranspileCtx) -> Result<Self::Output, TranspileError> {
        Ok(token_ref(Token::new(TokenType::Identifier {
            identifier: ShortString::new(self.to_string()),
        })))
    }
}
