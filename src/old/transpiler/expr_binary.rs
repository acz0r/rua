use full_moon::ast::{BinOp, Expression};

use crate::transpiler::{
    Transpile, TranspileWithoutCtx, and_token, expr::IntermediateExpr, greater_than_equal_token, greater_than_token, less_than_equal_token, less_than_token, minus_token, or_token, percent_token, plus_token, slash_token, star_token, tilde_equal_token, two_equal_token,
};

impl Transpile for syn::ExprBinary {
    type Output = IntermediateExpr;
    type Ctx = ();

    fn transpile_with_ctx(
        &self,
        _ctx: &(),
    ) -> Result<Self::Output, super::TranspileError> {
        let bin_op = match self.op {
            syn::BinOp::Add(_) => BinOp::Plus(plus_token()),
            syn::BinOp::Sub(_) => BinOp::Minus(minus_token()),
            syn::BinOp::Mul(_) => BinOp::Star(star_token()),
            syn::BinOp::Div(_) => BinOp::Slash(slash_token()),
            syn::BinOp::Rem(_) => BinOp::Percent(percent_token()),
            syn::BinOp::And(_) => BinOp::And(and_token()),
            syn::BinOp::Or(_) => BinOp::Or(or_token()),
            // syn::BinOp::BitXor(caret) => todo!(),
            // syn::BinOp::BitAnd(and) => todo!(),
            // syn::BinOp::BitOr(or) => todo!(),
            // syn::BinOp::Shl(shl) => todo!(),
            // syn::BinOp::Shr(shr) => todo!(),
            syn::BinOp::Eq(_) => BinOp::TwoEqual(two_equal_token()),
            syn::BinOp::Lt(_) => BinOp::LessThan(less_than_token()),
            syn::BinOp::Le(_) => BinOp::LessThanEqual(less_than_equal_token()),
            syn::BinOp::Ne(_) => BinOp::TildeEqual(tilde_equal_token()),
            syn::BinOp::Ge(_) => BinOp::GreaterThanEqual(greater_than_equal_token()),
            syn::BinOp::Gt(_) => BinOp::GreaterThan(greater_than_token()),
            syn::BinOp::AddAssign(_) => todo!(),
            syn::BinOp::SubAssign(_) => todo!(),
            syn::BinOp::MulAssign(_) => todo!(),
            syn::BinOp::DivAssign(_) => todo!(),
            syn::BinOp::RemAssign(_) => todo!(),
            syn::BinOp::BitXorAssign(_) => todo!(),
            syn::BinOp::BitAndAssign(_) => todo!(),
            syn::BinOp::BitOrAssign(_) => todo!(),
            syn::BinOp::ShlAssign(_) => todo!(),
            syn::BinOp::ShrAssign(_) => todo!(),
            _ => unimplemented!("Unsupported binary operator"),
        };

        Ok(IntermediateExpr::expr_only(Expression::BinaryOperator {
            lhs: Box::new(self.left.transpile()?.value_expr),
            binop: bin_op,
            rhs: Box::new(self.right.transpile()?.value_expr),
        }))
    }
}
