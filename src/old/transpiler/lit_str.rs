use full_moon::{
    ShortString,
    ast::Expression,
    tokenizer::{StringLiteralQuoteType, Token, TokenType},
};

use crate::transpiler::{Transpile, token_ref};

impl Transpile for syn::LitStr {
    type Output = Expression;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        Ok(Expression::String(token_ref(Token::new(
            TokenType::StringLiteral {
                literal: ShortString::new(escape_string(&self.value())),
                multi_line_depth: 0, // TODO: multiline?
                quote_type: StringLiteralQuoteType::Double,
            },
        ))))
    }
}

pub fn escape_string(s: &str) -> String {
    let mut escaped = String::new();

    for ch in s.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            ch if ch.is_ascii_control() => {
                use std::fmt::Write;
                write!(escaped, "\\{:03}", ch as u32).expect("writing to a String cannot fail");
            }
            ch => escaped.push(ch),
        }
    }

    escaped
}
