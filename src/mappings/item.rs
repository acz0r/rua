use crate::{Transpile, TranspileError, TranspileOptions, UniversalTranspileCtx, unsupported};
use full_moon::ast::Stmt;

pub mod item_const;
pub mod item_fn;

impl Transpile for syn::Item {
    type Ctx = ();
    // Some statements may not produce an actual Stmt in Lua, hence Option<Stmt> (e.g. enums)
    type Output = Option<Vec<Stmt>>;

    fn transpile(
        &self,
        _ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError> {
        Ok(match self {
            syn::Item::Const(item_const) => Some(item_const.transpile_default_ctx(u_ctx, opts)?),
            syn::Item::Enum(item_enum) => {
                // TODO: Register enum structure
                // u_ctx.register_enum(item_enum);
                todo!()
                // None
            }
            syn::Item::ExternCrate(item_extern_crate) => todo!(),
            syn::Item::Fn(item_fn) => Some(item_fn.transpile_default_ctx(u_ctx, opts)?),
            syn::Item::ForeignMod(item_foreign_mod) => todo!(),
            syn::Item::Impl(item_impl) => todo!(),
            syn::Item::Mod(item_mod) => todo!(),
            syn::Item::Static(item_static) => todo!(),
            syn::Item::Struct(item_struct) => todo!(),
            syn::Item::Trait(item_trait) => todo!(),
            syn::Item::TraitAlias(item_trait_alias) => todo!(),
            syn::Item::Type(item_type) => todo!(),
            syn::Item::Union(item_union) => todo!(),
            syn::Item::Use(item_use) => todo!(),
            syn::Item::Verbatim(token_stream) => todo!(),
            // Rust source should have macros pre-expanded before transpilation
            syn::Item::Macro(_) => unsupported!("macro"),
            _ => todo!(),
        })
    }
}
