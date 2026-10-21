use full_moon::{ShortString, ast::LocalAssignment, tokenizer::{Token, TokenType}};

use crate::transpiler::{Transpile, TranspileError, TranspileWithoutCtx, block::BlockCtx, eq_token, expr::IntermediateExpr, punctuate, token_ref};

impl Transpile for syn::ExprIf {
    type Output = IntermediateExpr;
    type Ctx = IfCtx;

    fn transpile_with_ctx(&self, ctx: &IfCtx) -> Result<Self::Output, TranspileError> {
        let cond = self.cond.transpile()?;
        let result_var = token_ref(Token::new(TokenType::Identifier { identifier: ShortString::from("_if_result") }));
        let then_branch = self.then_branch.transpile_with_ctx(&BlockCtx::new(result_var.clone()))?;
        let else_branch = if let Some((_, else_expr)) = &self.else_branch {
            Some(match &**else_expr {
                syn::Expr::Block(else_block) => {
                    // Else block
                    else_block.block.transpile_with_ctx(&BlockCtx::new(result_var.clone()))
                },
                syn::Expr::If(expr_else_if) => {
                    // Else if
                    todo!()
                },
                _ => todo!(),
            })
        } else {
            None
        };

        let local = LocalAssignment::new(punctuate(vec![result_var.clone()]));


        // let cond = self.cond.transpile()?;
        // let then_branch = if ctx.is_assignment {
        //     self.then_branch.transpile_with_ctx(&BlockCtx::new(BlockDest::Assign))?
        // } else {
        //     self.then_branch.transpile()?
        // };
        // let else_branch = if let Some((_, else_expr)) = &self.else_branch {
        //     Some(else_expr.transpile()?)
        // } else {
        //     None
        // };

        // TODO: Turn if stmt into value-producing statement

        /*
        if x {
            ...
        } else {
            ...
        }

        let y = if x {
        ...
        } else {
         ...
        }

        if no assignment, then expression does nothing, but there's a prior statement that contains the if statement
         */

        Ok(TranspiledExpr::expr_only(expr))
    }
}

pub struct IfCtx {
    pub is_assignment: bool,
}
