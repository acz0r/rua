use full_moon::{
    ShortString,
    ast::Expression,
    tokenizer::{Token, TokenType},
};

use crate::transpiler::{Transpile, token_ref};

impl Transpile for syn::LitInt {
    type Output = Expression;

    fn transpile(
        &self,
        _ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        Ok(Expression::Number(token_ref(Token::new(
            TokenType::Number {
                text: ShortString::new(self.base10_digits()),
            },
        ))))
    }
}
