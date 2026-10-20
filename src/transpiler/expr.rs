use crate::transpiler::{Transpile, stmt::TranspiledExpr};

impl Transpile for syn::Expr {
    type Output = TranspiledExpr;

    fn transpile(
        &self,
        ctx: &mut super::TranspileCtx,
    ) -> Result<Self::Output, super::TranspileError> {
        match self {
            syn::Expr::Array(expr_array) => todo!(),
            syn::Expr::Assign(expr_assign) => todo!(),
            // syn::Expr::Async(expr_async) => todo!(),
            // syn::Expr::Await(expr_await) => todo!(),
            syn::Expr::Binary(expr_binary) => Ok(expr_binary.transpile(ctx)?),
            syn::Expr::Block(expr_block) => todo!(),
            syn::Expr::Break(expr_break) => todo!(),
            syn::Expr::Call(expr_call) => todo!(),
            syn::Expr::Cast(expr_cast) => todo!(),
            syn::Expr::Closure(expr_closure) => todo!(),
            syn::Expr::Const(expr_const) => todo!(),
            syn::Expr::Continue(expr_continue) => todo!(),
            syn::Expr::Field(expr_field) => todo!(),
            syn::Expr::ForLoop(expr_for_loop) => todo!(),
            syn::Expr::Group(expr_group) => todo!(),
            syn::Expr::If(expr_if) => Ok(expr_if.transpile(ctx)?),
            syn::Expr::Index(expr_index) => todo!(),
            syn::Expr::Infer(expr_infer) => todo!(),
            syn::Expr::Let(expr_let) => todo!(),
            syn::Expr::Lit(expr_lit) => Ok(expr_lit.transpile(ctx)?),
            syn::Expr::Loop(expr_loop) => todo!(),
            // syn::Expr::Macro(expr_macro) => todo!(),
            syn::Expr::Match(expr_match) => todo!(),
            syn::Expr::MethodCall(expr_method_call) => todo!(),
            syn::Expr::Paren(expr_paren) => todo!(),
            syn::Expr::Path(expr_path) => Ok(expr_path.transpile(ctx)?),
            syn::Expr::Range(expr_range) => todo!(),
            // syn::Expr::RawAddr(expr_raw_addr) => todo!(),
            syn::Expr::Reference(expr_reference) => todo!(),
            syn::Expr::Repeat(expr_repeat) => todo!(),
            syn::Expr::Return(expr_return) => todo!(),
            syn::Expr::Struct(expr_struct) => todo!(),
            syn::Expr::Try(expr_try) => todo!(),
            syn::Expr::TryBlock(expr_try_block) => todo!(),
            syn::Expr::Tuple(expr_tuple) => todo!(),
            syn::Expr::Unary(expr_unary) => Ok(expr_unary.transpile(ctx)?),
            // syn::Expr::Unsafe(expr_unsafe) => todo!(),
            // syn::Expr::Verbatim(token_stream) => todo!(),
            syn::Expr::While(expr_while) => todo!(),
            // syn::Expr::Yield(expr_yield) => todo!(),
            _ => unimplemented!("Unsupported expression type"),
        }
    }
}
