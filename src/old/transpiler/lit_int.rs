use full_moon::{
    ShortString,
    ast::Expression,
    tokenizer::{Token, TokenType},
};

use crate::transpiler::{Transpile, token_ref};

impl Transpile for syn::LitInt {
    type Output = Expression;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        Ok(Expression::Number(token_ref(Token::new(
            TokenType::Number {
                text: ShortString::new(self.base10_digits()),
            },
        ))))
    }
}
