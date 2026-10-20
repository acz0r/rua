use full_moon::{
    ShortString,
    ast::{Expression, Var},
    tokenizer::{Token, TokenType},
};

use crate::transpiler::{Transpile, stmt::TranspiledExpr, token_ref};

impl Transpile for syn::ExprPath {
    type Output = TranspiledExpr;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        // Ignore qself (for now)
        let path = &self.path;

        // TODO: support more complex path variants
        let first = path.get_ident().unwrap();
        Ok(TranspiledExpr::expr_only(Expression::Var(Var::Name(
            token_ref(Token::new(TokenType::Identifier {
                identifier: ShortString::new(first.to_string()),
            })),
        ))))
    }
}
