use full_moon::ast::{FunctionBody, LocalFunction, Parameter, Stmt};

use crate::transpiler::{Transpile, comma_separate};

impl Transpile for syn::ItemFn {
    type Output = Stmt;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        let fn_ident = self.sig.ident.transpile(ctx)?;
        let params = comma_separate(
            self.sig
                .inputs
                .iter()
                .map(|i| i.transpile(ctx).map(|t| Parameter::Name(t)))
                .collect::<Result<Vec<_>, _>>()?,
        );
        let body = FunctionBody::new()
            .with_parameters(params)
            .with_block(self.block.transpile(ctx)?);
        let decl = LocalFunction::new(fn_ident).with_body(body);

        Ok(Stmt::LocalFunction(Box::new(decl)))
    }
}
