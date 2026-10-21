use crate::transpiler::{Transpile, TranspileError, TranspileWithoutCtx};
use full_moon::ast::Stmt;

impl Transpile for &syn::Item {
    type Output = Stmt;
    type Ctx = ();

    fn transpile_with_ctx(&self, _ctx: &()) -> Result<Self::Output, TranspileError> {
        match self {
            syn::Item::Const(item_const) => Ok(item_const.transpile()?),
            syn::Item::Enum(item_enum) => todo!(),
            // syn::Item::ExternCrate(item_extern_crate) => todo!(),
            syn::Item::Fn(item_fn) => Ok(item_fn.transpile()?),
            // syn::Item::ForeignMod(item_foreign_mod) => todo!(),
            syn::Item::Impl(item_impl) => todo!(),
            // TODO: syn::Item::Macro(item_macro) => todo!(),
            syn::Item::Mod(item_mod) => todo!(),
            syn::Item::Static(item_static) => todo!(),
            syn::Item::Struct(item_struct) => todo!(),
            syn::Item::Trait(item_trait) => todo!(),
            syn::Item::TraitAlias(item_trait_alias) => todo!(),
            syn::Item::Type(item_type) => todo!(),
            syn::Item::Union(item_union) => todo!(),
            // syn::Item::Use(item_use) => todo!(),
            // syn::Item::Verbatim(item_verbatim) => todo!(),
            _ => unimplemented!("Unsupported item type"),
        }
    }
}
