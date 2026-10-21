use full_moon::{ShortString, ast::{Block, Do, Expression, LocalAssignment, Stmt, Var}, tokenizer::{Token, TokenType}};

use crate::transpiler::{Transpile, TranspileError, TranspileWithoutCtx, eq_token, expr::IntermediateExpr, punctuate, token_ref};

impl Transpile for syn::Stmt {
    type Output = Vec<Stmt>;
    type Ctx = ();

    fn transpile_with_ctx(&self, _ctx: &()) -> Result<Self::Output, TranspileError> {
        match self {
            syn::Stmt::Local(local) => todo!(),
            syn::Stmt::Item(item) => todo!(),

            syn::Stmt::Expr(expr, _) => {
                let IntermediateExpr { mut prelude, value_expr } = expr.transpile()?;

                prelude.extend(expr_to_stmts(value_expr)?);
                Ok(prelude)
            }

            syn::Stmt::Macro(_) => todo!(),
        }
    }
}

fn expr_to_stmts(value_expr: Expression) -> Result<Vec<Stmt>, TranspileError> {
    match value_expr {
        Expression::FunctionCall(function_call) => Ok(vec![Stmt::FunctionCall(function_call)]),
        Expression::Number(_) | Expression::String(_) | Expression::Symbol(_) | Expression::Var(Var::Name(_)) => Ok(vec![]),
        other => {
            // Discard value
            let unused_var = token_ref(Token::new(TokenType::Identifier { identifier: ShortString::from("_") }));
            let local = LocalAssignment::new(punctuate(vec![unused_var]))
                .with_equal_token(Some(eq_token()))
                .with_expressions(punctuate(vec![other]));

            let block = Block::new().with_stmts(vec![(Stmt::LocalAssignment(local), None)]);

            Ok(vec![Stmt::Do(Box::new(Do::new().with_block(block)))])
        },
    }
}
