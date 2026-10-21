use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx};
use full_moon::ast::{Expression, Stmt, Var};

pub mod expr_binary;
pub mod expr_lit;
pub mod expr_path;
pub mod expr_unary;

impl Transpile for syn::Expr {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(match self {
            syn::Expr::Array(expr_array) => todo!(),
            syn::Expr::Assign(expr_assign) => todo!(),
            syn::Expr::Async(expr_async) => todo!(),
            syn::Expr::Await(expr_await) => todo!(),
            syn::Expr::Binary(expr_binary) => expr_binary.transpile_default_ctx(u_ctx, opts)?,
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
            syn::Expr::If(expr_if) => todo!(),
            syn::Expr::Index(expr_index) => todo!(),
            syn::Expr::Infer(expr_infer) => todo!(),
            syn::Expr::Let(expr_let) => todo!(),
            syn::Expr::Lit(expr_lit) => expr_lit.transpile_default_ctx(u_ctx, opts)?,
            syn::Expr::Loop(expr_loop) => todo!(),
            syn::Expr::Macro(expr_macro) => todo!(),
            syn::Expr::Match(expr_match) => todo!(),
            syn::Expr::MethodCall(expr_method_call) => todo!(),
            syn::Expr::Paren(expr_paren) => todo!(),
            syn::Expr::Path(expr_path) => expr_path.transpile_default_ctx(u_ctx, opts)?,
            syn::Expr::Range(expr_range) => todo!(),
            syn::Expr::RawAddr(expr_raw_addr) => todo!(),
            syn::Expr::Reference(expr_reference) => todo!(),
            syn::Expr::Repeat(expr_repeat) => todo!(),
            syn::Expr::Return(expr_return) => todo!(),
            syn::Expr::Struct(expr_struct) => todo!(),
            syn::Expr::Try(expr_try) => todo!(),
            syn::Expr::TryBlock(expr_try_block) => todo!(),
            syn::Expr::Tuple(expr_tuple) => todo!(),
            syn::Expr::Unary(expr_unary) => expr_unary.transpile_default_ctx(u_ctx, opts)?,
            syn::Expr::Unsafe(expr_unsafe) => todo!(),
            syn::Expr::Verbatim(token_stream) => todo!(),
            syn::Expr::While(expr_while) => todo!(),
            syn::Expr::Yield(expr_yield) => todo!(),
            _ => todo!(),
        })
    }
}

pub enum IntermediateExpr {
    // Output is simple enough to be written as a single Lua expression
    Simple(Expression),
    // Output needed 1 or more leading Lua statements to reach parity with the Rust code and the output was bound to this variable, caller is expected to use at their discretion
    Bound(BoundExpr),
}

impl IntermediateExpr {
    pub fn bound(leading_stmts: Vec<Stmt>, expr: Expression) -> Self {
        IntermediateExpr::Bound(BoundExpr {
            leading_stmts,
            expr,
        })
    }

    pub fn split(self) -> (Vec<Stmt>, Expression) {
        match self {
            IntermediateExpr::Simple(expr) => (vec![], expr),
            IntermediateExpr::Bound(bound_expr) => (bound_expr.leading_stmts, bound_expr.expr),
        }
    }

    pub fn expand(self, additional_expr: IntermediateExpr) -> IntermediateExpr {
        todo!()
    }
}

pub struct BoundExpr {
    pub expr: Expression,
    pub leading_stmts: Vec<Stmt>,
}
