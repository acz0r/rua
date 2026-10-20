use full_moon::ast::{Expression, Stmt};

use crate::transpiler::{Transpile, TranspileCtx, TranspileError};

impl Transpile for syn::Stmt {
    type Output = Vec<Stmt>;

    fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
        match self {
            syn::Stmt::Local(local) => todo!(),
            syn::Stmt::Item(item) => todo!(),

            syn::Stmt::Expr(expr, _) => {
                let TranspiledExpr { mut prelude, value } = expr.transpile(ctx)?;

                prelude.extend(discard_value(value, ctx)?);
                Ok(prelude)
            }

            syn::Stmt::Macro(_) => todo!(),
        }
    }
}

pub struct TranspiledExpr {
    pub prelude: Vec<Stmt>,
    pub value: Expression,
}

impl TranspiledExpr {
    pub fn new(prelude: Vec<Stmt>, value: Expression) -> Self {
        Self { prelude, value }
    }

    pub fn expr_only(expr: Expression) -> Self {
        Self {
            prelude: vec![],
            value: expr,
        }
    }

    pub fn prelude(&self) -> &Vec<Stmt> {
        &self.prelude
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

pub fn discard_value(
    expr: Expression,
    _ctx: &mut TranspileCtx,
) -> Result<Vec<Stmt>, TranspileError> {
    // Implement the logic to discard the value of an expression, if necessary.
    // This is a placeholder implementation.

    Ok(vec![])
}
