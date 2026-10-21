use full_moon::{
    ast::{Block, Expression},
    tokenizer::Symbol,
};

use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx, symbol};
impl Transpile for syn::Block {
    type Ctx = ();
    type Output = IntermediateBlock;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        // TODO: If block divergence
        let mut stmts = Vec::new();

        for (index, stmt) in self.stmts.iter().enumerate() {
            let is_last = index + 1 == self.stmts.len();

            if let syn::Stmt::Expr(expr, None) = stmt {
                // Explicit exits need statement/control-flow lowering,
                // even when written without a semicolon.
                let is_exit = matches!(
                    expr,
                    syn::Expr::Return(_) | syn::Expr::Break(_) | syn::Expr::Continue(_)
                );

                if is_last && !is_exit {
                    let (leading_stmts, value) = expr.transpile_default_ctx(u_ctx, opts)?.split();

                    stmts.extend(leading_stmts.into_iter().map(|stmt| (stmt, None)));

                    return Ok(IntermediateBlock::new(
                        Block::new().with_stmts(stmts),
                        BlockResult::Value(value),
                    ));
                }
            }

            let intermediate = stmt.transpile_default_ctx(u_ctx, opts)?;

            stmts.extend(intermediate.stmt.into_iter().map(|stmt| (stmt, None)));

            if let Some(last) = intermediate.last {
                return Ok(IntermediateBlock::new(
                    Block::new()
                        .with_stmts(stmts)
                        .with_last_stmt(Some((last, None))),
                    BlockResult::Diverges,
                ));
            }
        }

        Ok(IntermediateBlock::new(
            Block::new().with_stmts(stmts),
            BlockResult::Value(Expression::Symbol(symbol(Symbol::Nil))),
        ))
    }
}

pub struct IntermediateBlock {
    pub block: Block,
    pub result: BlockResult,
}

impl IntermediateBlock {
    pub fn new(block: Block, result: BlockResult) -> Self {
        Self { block, result }
    }
}

pub enum BlockResult {
    Value(Expression),
    Diverges,
}
