use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx};
use full_moon::ast::{Ast, Block, Stmt};

impl Transpile for syn::File {
    type Ctx = ();
    type Output = Ast;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        let mut ast = full_moon::parse("").unwrap();

        // Ignore shebang
        // Ignore frontmatter
        // Ignore inner attrs (for now)

        // Parse items
        let stmts: Vec<Stmt> = self
            .items
            .iter()
            .filter_map(|item| {
                match item.transpile_default_ctx(u_ctx, opts) {
                    Ok(Some(val)) => Some(Ok(val)),
                    Ok(None) => None, // Drop no-op stmts
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        let block = Block::new().with_stmts(stmts.into_iter().map(|s| (s, None)).collect());

        ast = ast.with_nodes(block);
        Ok(ast)
    }
}
