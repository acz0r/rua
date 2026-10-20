use full_moon::{
    ast::Expression,
    tokenizer::{Symbol, Token, TokenType},
};

use crate::transpiler::{Transpile, token_ref};

impl Transpile for syn::LitBool {
    type Output = Expression;

    fn transpile(
        &self,
        _ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        let symbol = match self.value() {
            true => Symbol::True,
            false => Symbol::False,
        };

        Ok(Expression::Symbol(token_ref(Token::new(
            TokenType::Symbol { symbol: symbol },
        ))))
    }
}
