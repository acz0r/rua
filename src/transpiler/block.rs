use full_moon::{
    ast::{Block, LastStmt, Return, Stmt},
    tokenizer::TokenReference,
};

use crate::transpiler::{Transpile, TranspileCtx, comma_separate, nil};

fn transpile_block(
    block: &syn::Block,
    dest: BlockDest,
    ctx: &mut TranspileCtx,
) -> Result<Block, super::TranspileError> {
    let mut out = BlockOutput::default();

    for (index, stmt) in block.stmts.iter().enumerate() {
        let is_last = index + 1 == block.stmts.len();

        match stmt {
            syn::Stmt::Expr(expr, semi) => {
                let is_tail = is_last && semi.is_none();

                let expr_dest = if is_tail {
                    dest.clone()
                } else {
                    BlockDest::Discard
                };

                lower_expr(expr, expr_dest, &mut out, ctx)?;

                if out.last_stmt.is_some() || is_tail {
                    return Ok(out.finish());
                }
            }
            _ => out.stmts.extend(stmt.transpile(ctx)?),
        }
    }

    complete_block(nil(), dest, &mut out, ctx)?;

    Ok(out.finish())
}

fn lower_expr(
    expr: &syn::Expr,
    dest: BlockDest,
    out: &mut BlockOutput,
    ctx: &mut TranspileCtx,
) -> Result<(), super::TranspileError> {
    todo!()
}

fn complete_block(
    value: TokenReference,
    dest: BlockDest,
    out: &mut BlockOutput,
    ctx: &mut TranspileCtx,
) -> Result<(), super::TranspileError> {
    todo!()
}

// A builder that handles full_moon's separation of Stmt and LastStmt.
#[derive(Default)]
struct BlockOutput {
    stmts: Vec<Stmt>,
    last_stmt: Option<LastStmt>,
}

impl BlockOutput {
    fn finish(self) -> Block {
        Block::new()
            .with_stmts(self.stmts.into_iter().map(|stmt| (stmt, None)).collect())
            .with_last_stmt(self.last_stmt.map(|stmt| (stmt, None)))
    }
}

// impl Transpile for syn::Block {
//     type Output = TranspiledBlock;

//     fn transpile(
//         &self,
//         ctx: &mut super::TranspileCtx,
//     ) -> Result<Self::Output, super::TranspileError> {
//         let mut stmts = Vec::new();
//         let mut last_stmt = None;

//         for (index, stmt) in self.stmts.iter().enumerate() {
//             let is_last = index + 1 == self.stmts.len();

//             match stmt {
//                 // Explicit return, with or without a semicolon.
//                 syn::Stmt::Expr(syn::Expr::Return(ret), _) => {
//                     let values = ret
//                         .expr
//                         .as_ref()
//                         .map(|expr| expr.transpile(ctx).map(|e| e.value))
//                         .transpose()?
//                         .into_iter()
//                         .collect();

//                     last_stmt = Some((
//                         LastStmt::Return(Return::new().with_returns(comma_separate(values))),
//                         None,
//                     ));
//                     break; // Remaining Rust statements are unreachable.
//                 }

//                 // Tail expression: `fn foo() -> i32 { 42 }`.
//                 syn::Stmt::Expr(expr, None) if is_last => {
//                     let value = expr.transpile(ctx)?.value;
//                     last_stmt = Some((
//                         LastStmt::Return(Return::new().with_returns(comma_separate(vec![value]))),
//                         None,
//                     ));
//                 }

//                 _ => {
//                     for s in stmt.transpile(ctx)? {
//                         stmts.push((s, None));
//                     }
//                 }
//             }
//         }

//         Ok(Block::new().with_stmts(stmts).with_last_stmt(last_stmt))
//     }
// }

#[derive(Debug, Clone)]
pub enum BlockDest {
    Discard,
    Assign,
    Return,
}
