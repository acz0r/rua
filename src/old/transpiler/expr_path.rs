use full_moon::{
    ShortString,
    ast::{Expression, Var},
    tokenizer::{Token, TokenType},
};

use crate::transpiler::{Transpile, expr::IntermediateExpr, token_ref};

impl Transpile for syn::ExprPath {
    type Output = IntermediateExpr;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        // Ignore qself (for now)
        let path = &self.path;

        // TODO: support more complex path variants
        let first = path.get_ident().unwrap();
        Ok(IntermediateExpr::expr_only(Expression::Var(Var::Name(
            token_ref(Token::new(TokenType::Identifier {
                identifier: ShortString::new(first.to_string()),
            })),
        ))))
    }
}
