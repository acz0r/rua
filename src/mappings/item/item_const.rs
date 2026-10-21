use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx, single, symbol};
use full_moon::ast::LocalAssignment;
use full_moon::ast::Stmt;
use full_moon::tokenizer::Symbol;

impl Transpile for syn::ItemConst {
    type Ctx = ();
    type Output = Vec<Stmt>;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        // Ignore attrs
        // Ignore vis
        // Ignore modifiers
        // Ignore const token
        // Ignore generics
        // Ignore colon token
        // Ignore type
        // Ignore equal token
        // Ignore semi token

        // Transpile ident
        let ident = self.ident.transpile_default_ctx(u_ctx, opts)?;

        // Transpile expr
        let (mut stmts, expr) = self.expr.transpile_default_ctx(u_ctx, opts)?.split();

        stmts.push(Stmt::LocalAssignment(
            LocalAssignment::new(single(ident))
                .with_equal_token(Some(symbol(Symbol::Equal)))
                .with_expressions(single(expr)),
        ));
        Ok(stmts)
    }
}
