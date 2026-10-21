use crate::transpiler::{Transpile, TranspileError, TranspileWithoutCtx};
use full_moon::ast::{Ast, Block};

impl Transpile for syn::File {
    type Output = Ast;
    type Ctx = ();

    fn transpile_with_ctx(&self, _ctx: &()) -> Result<Self::Output, TranspileError> {
        let mut ast = full_moon::parse("").unwrap();

        // Ignore shebang
        // Ignore frontmatter
        // Ignore inner attrs (for now)

        // Parse items
        let stmts = self
            .items
            .iter()
            .map(|item| item.transpile())
            .collect::<Result<Vec<_>, _>>()?;

        let block = Block::new().with_stmts(stmts.into_iter().map(|s| (s, None)).collect());

        ast = ast.with_nodes(block);
        Ok(ast)
    }
}
