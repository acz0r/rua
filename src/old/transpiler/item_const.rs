use crate::transpiler::{Transpile, TranspileError, TranspileWithoutCtx, eq_token, punctuate};
use full_moon::ast::{LocalAssignment, Stmt};

impl Transpile for syn::ItemConst {
    type Output = Stmt;
    type Ctx = ();

    fn transpile_with_ctx(&self, _ctx: &()) -> Result<Self::Output, TranspileError> {
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
        let ident = self.ident.transpile()?;

        // Transpile expr
        let expr = self.expr.transpile()?;

        Ok(Stmt::LocalAssignment(
            LocalAssignment::new(punctuate(vec![ident]))
                .with_equal_token(Some(eq_token()))
                .with_expressions(punctuate(vec![expr.value_expr])),
        ))
    }
}
