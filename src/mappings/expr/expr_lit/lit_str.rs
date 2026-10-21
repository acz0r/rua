use crate::mappings::expr::IntermediateExpr;
use crate::{Transpile, str_lit};
use crate::{TranspileError, TranspileOptions, UniversalTranspileCtx};
use std::fmt::Write;
impl Transpile for syn::LitStr {
    type Ctx = ();
    type Output = IntermediateExpr;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        _u_ctx: &mut UniversalTranspileCtx,
        _opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(IntermediateExpr::Simple(str_lit(escape(self.value()))))
    }
}

pub fn escape(s: impl AsRef<str>) -> String {
    let mut escaped = String::new();

    for ch in s.as_ref().chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            ch if ch.is_ascii_control() => {
                write!(escaped, "\\{:03}", ch as u32).expect("writing to a String cannot fail");
            }
            ch => escaped.push(ch),
        }
    }

    escaped
}
