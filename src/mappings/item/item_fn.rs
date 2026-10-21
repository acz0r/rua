use full_moon::ast::{FunctionBody, LastStmt, LocalFunction, Parameter, Return, Stmt};

use crate::{
    Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx, comma_separated, ident,
    mappings::block::BlockResult, single, unsupported,
};
impl Transpile for syn::ItemFn {
    type Ctx = ();
    type Output = Vec<Stmt>;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        let sig = &self.sig;
        let ident = ident(sig.ident.to_string());

        if sig.variadic.is_some() {
            unsupported!("variadic function");
        }

        let mut parameters = Vec::new();

        for input in &sig.inputs {
            let typed = match input {
                syn::FnArg::Typed(typed) => typed,
                syn::FnArg::Receiver(_) => {
                    unsupported!("function receiver");
                }
            };

            let binding = match typed.pat.as_ref() {
                syn::Pat::Ident(binding)
                    if binding.by_ref.is_none() && binding.subpat.is_none() =>
                {
                    binding
                }
                _ => {
                    unsupported!(
                        "function parameter pattern",
                        "only plain identifier bindings are supported"
                    );
                }
            };

            let name = binding.ident.transpile_default_ctx(u_ctx, opts)?;
            parameters.push(Parameter::Name(name));
        }

        let intermediate_block = self.block.transpile_default_ctx(u_ctx, opts)?;

        let block = match intermediate_block.result {
            BlockResult::Value(value) => intermediate_block.block.with_last_stmt(Some((
                LastStmt::Return(Return::new().with_returns(single(value))),
                None,
            ))),
            BlockResult::Diverges => intermediate_block.block,
        };

        let body = FunctionBody::new()
            .with_parameters(comma_separated(parameters))
            .with_block(block);
        let fn_def = LocalFunction::new(ident).with_body(body);

        Ok(vec![Stmt::LocalFunction(Box::new(fn_def))])
    }
}
