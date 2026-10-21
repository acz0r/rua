use full_moon::{
    ShortString,
    ast::Expression,
    tokenizer::{StringLiteralQuoteType, Token, TokenType},
};

use crate::transpiler::{Transpile, lit_str::escape_string, token_ref};

impl Transpile for syn::LitChar {
    type Output = Expression;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        Ok(Expression::String(token_ref(Token::new(
            TokenType::StringLiteral {
                literal: ShortString::new(escape_string(&String::from(self.value()))),
                multi_line_depth: 0, // TODO: multiline?
                // StyLua will change this to double quotes, but intention
                // is to have char literals become single-quoted string literals
                quote_type: StringLiteralQuoteType::Single,
            },
        ))))
    }
}
