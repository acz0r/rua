use full_moon::ast::{LastStmt, Stmt};

use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx};
impl Transpile for syn::Stmt {
    type Ctx = ();
    type Output = IntermediateStmt;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut UniversalTranspileCtx,
        _opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        todo!()
    }
}

pub struct IntermediateStmt {
    pub stmt: Vec<Stmt>,
    pub last: Option<LastStmt>,
}
