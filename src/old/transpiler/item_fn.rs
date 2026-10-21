use full_moon::{ShortString, ast::{Expression, FunctionBody, LastStmt, LocalFunction, Parameter, Return, Stmt, Var}, tokenizer::{Token, TokenType}};

use crate::transpiler::{Transpile, TranspileWithoutCtx, block::BlockCtx, comma_separate, punctuate, token_ref};

impl Transpile for syn::ItemFn {
    type Output = Stmt;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        let fn_ident = self.sig.ident.transpile()?;
        let params = comma_separate(
            self.sig
                .inputs
                .iter()
                .map(|i| i.transpile().map(|t| Parameter::Name(t)))
                .collect::<Result<Vec<_>, _>>()?,
        );
        let return_var = token_ref(Token::new(TokenType::Identifier { identifier: ShortString::from("_return") }));
        let mut block = self.block.transpile_with_ctx(&BlockCtx::new(return_var.clone()))?;
        let ret = Return::new().with_returns(punctuate(vec![Expression::Var(Var::Name(return_var))]));
        block = block.with_last_stmt(Some((LastStmt::Return(ret), None)));
        let body = FunctionBody::new()
            .with_parameters(params)
            .with_block(block);
        let decl = LocalFunction::new(fn_ident).with_body(body);

        Ok(Stmt::LocalFunction(Box::new(decl)))
    }
}
