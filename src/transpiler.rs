use std::fmt::Display;

use full_moon::{
    ast::punctuated::{Pair, Punctuated},
    tokenizer::{Symbol, Token, TokenReference, TokenType},
};
use stylua_lib::{OutputVerification, QuoteStyle};
use thiserror::Error;

mod block;
mod expr;
mod expr_binary;
mod expr_if;
mod expr_lit;
mod expr_path;
mod expr_unary;
mod file;
mod fn_arg;
mod ident;
mod item;
mod item_const;
mod item_fn;
mod lit_bool;
mod lit_char;
mod lit_float;
mod lit_int;
mod lit_str;
mod pat;
mod stmt;

pub fn transpile(rust_src: &str) -> Result<String, TranspileError> {
    let rust_ast = syn::parse_file(rust_src)?;

    let mut ctx = TranspileCtx::default();
    let lua_ast = rust_ast.transpile(&mut ctx)?;

    let fmt_config = stylua_lib::Config {
        column_width: usize::MAX,
        quote_style: QuoteStyle::ForceDouble,
        ..Default::default()
    };

    let formatted_lua_ast =
        stylua_lib::format_ast(lua_ast, fmt_config, None, OutputVerification::None)?;

    Ok(formatted_lua_ast.to_string())
}

pub fn token_ref(token: Token) -> TokenReference {
    TokenReference::new(vec![], token, vec![])
}

pub fn eq_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Equal,
    }))
}

pub fn minus_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Minus,
    }))
}

pub fn punctuate<T: Display>(tokens: Vec<T>) -> Punctuated<T> {
    let mut punctuated = Punctuated::new();
    for token in tokens {
        punctuated.push(Pair::new(token, None));
    }
    punctuated
}

pub fn comma_separate<T: Display>(tokens: Vec<T>) -> Punctuated<T> {
    let mut punctuated = Punctuated::new();
    let len = tokens.len();
    for (i, token) in tokens.into_iter().enumerate() {
        let punct = if i < len - 1 {
            Some(Token::new(TokenType::Symbol {
                symbol: Symbol::Comma,
            }))
        } else {
            None
        };
        punctuated.push(Pair::new(token, punct.map(|p| token_ref(p))));
    }
    punctuated
}

pub fn plus_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Plus,
    }))
}

pub fn star_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Star,
    }))
}

pub fn slash_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Slash,
    }))
}

pub fn percent_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Percent,
    }))
}

pub fn and_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::And,
    }))
}

pub fn or_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol { symbol: Symbol::Or }))
}

pub fn two_equal_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::TwoEqual,
    }))
}

pub fn less_than_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::LessThan,
    }))
}

pub fn less_than_equal_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::LessThanEqual,
    }))
}

pub fn tilde_equal_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::TildeEqual,
    }))
}

pub fn greater_than_equal_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::GreaterThanEqual,
    }))
}

pub fn greater_than_token() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::GreaterThan,
    }))
}

pub fn nil() -> TokenReference {
    token_ref(Token::new(TokenType::Symbol {
        symbol: Symbol::Nil,
    }))
}

pub trait Transpile {
    type Output;

    fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError>;
}

#[derive(Debug)]
pub struct TranspileCtx {
    pub in_global_scope: bool,
}

impl Default for TranspileCtx {
    fn default() -> Self {
        Self {
            in_global_scope: true,
        }
    }
}

#[derive(Error, Debug)]
pub enum TranspileError {
    #[error("Failed to parse Rust source: {0}")]
    ParseError(#[from] syn::Error),
    #[error("StyLua formatting error: {0}")]
    StyLuaError(#[from] stylua_lib::Error),
    // #[error("Unsupported Rust language feature: {rust_lang_feature}")]
    // UnsupportedRustFeature { rust_lang_feature: &'static str },
}

// macro_rules! unsupported_rust_feat {
//     ($feature:expr) => {
//         return Err(TranspileError::UnsupportedRustFeature {
//             rust_lang_feature: $feature,
//         })
//     };
// }

// // `Vec<Item>` -> `Vec<Stmt>`
// impl Transpile for syn::Item {
//     // A Lua stmt followed by an optional semicolon
//     type Output = LuaStmt;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         match self {
//             syn::Item::Const(item_const) => item_const.transpile(ctx),
//             syn::Item::Enum(item_enum) => todo!(),
//             syn::Item::ExternCrate(item_extern_crate) => todo!(),
//             syn::Item::Fn(item_fn) => Ok(LuaStmt::Do(Box::new(
//                 LuaDo::new().with_block(
//                     LuaBlock::new().with_stmts(
//                         item_fn
//                             .block
//                             .stmts
//                             .iter()
//                             .map(|s| s.transpile(ctx).map(|s| (s, None)))
//                             .collect::<Result<Vec<_>, _>>()?,
//                     ),
//                 ),
//             ))),
//             syn::Item::ForeignMod(item_foreign_mod) => todo!(),
//             syn::Item::Impl(item_impl) => todo!(),
//             syn::Item::Macro(item_macro) => todo!(),
//             syn::Item::Mod(item_mod) => todo!(),
//             syn::Item::Static(item_static) => todo!(),
//             syn::Item::Struct(item_struct) => todo!(),
//             syn::Item::Trait(item_trait) => todo!(),
//             syn::Item::TraitAlias(item_trait_alias) => todo!(),
//             syn::Item::Type(item_type) => todo!(),
//             syn::Item::Union(item_union) => todo!(),
//             syn::Item::Use(item_use) => todo!(),
//             syn::Item::Verbatim(token_stream) => todo!(),
//             _ => unsupported_rust_feat!("[unknown] item"),
//         }
//     }
// }

// impl Transpile for syn::ExprArray {
//     type Output = LuaTableConstructor;
//             .enumerate()
//             .map(|(index, elem)| {
//                 // Ensures all lua expressions resolve to a value
//                 let value_expr = transpile_value_expr(elem, ctx)?;
//                 let comma = (index + 1 < self.elems.len())
//                     .then(|| TokenReference::symbol(", ").expect("comma is a valid Lua symbol"));

//                 Ok(LuaPair::new(LuaField::NoKey(value_expr), comma))
//             })
//             .collect::<Result<full_moon::ast::punctuated::Punctuated<LuaField>, TranspileError>>(
//             )?;

//         Ok(LuaTableConstructor::new().with_fields(fields))
//     }
// }

// impl Transpile for syn::Stmt {
//     type Output = LuaStmt;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         match self {
//             syn::Stmt::Local(local) => Ok(LuaStmt::LocalAssignment(local.transpile(ctx)?)),
//             syn::Stmt::Item(item) => item.transpile(ctx),
//             syn::Stmt::Expr(expr, semi) => todo!(),
//             syn::Stmt::Macro(stmt_macro) => todo!(),
//         }
//     }
// }

// impl Transpile for syn::Local {
//     type Output = LuaLocalAssignment;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         match &self.pat {
//             syn::Pat::Ident(pat_ident) => {
//                 let var_name = transpile_pat_ident(&pat_ident, ctx)?;

//                 let mut name_list = LuaPunctuated::new();
//                 name_list.push(LuaPair::new(var_name, None));

//                 let mut expressions_list = LuaPunctuated::new();
//                 let equals_token = match &self.init {
//                     Some(init) => {
//                         let expr = init.transpile(ctx)?;
//                         expressions_list.push(LuaPair::new(expr, None));

//                         Some(TokenReference::new(
//                             vec![],
//                             Token::new(TokenType::Symbol {
//                                 symbol: LuaSymbol::Equal,
//                             }),
//                             vec![],
//                         ))
//                     }
//                     None => None,
//                 };

//                 Ok(LuaLocalAssignment::new(name_list)
//                     .with_equal_token(equals_token)
//                     .with_expressions(expressions_list))
//             }
//             _ => unimplemented!(),
//         }
//     }
// }

// impl Transpile for syn::LocalInit {
//     type Output = LuaExpression;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         transpile_value_expr(&self.expr, ctx)
//     }
// }

// fn transpile_pat_ident(
//     pat_ident: &syn::PatIdent,
//     ctx: &mut TranspileCtx,
// ) -> Result<TokenReference, TranspileError> {
//     Ok(TokenReference::new(
//         vec![],
//         Token::new(TokenType::Identifier {
//             identifier: ShortString::new(pat_ident.ident.to_string()),
//         }),
//         vec![],
//     ))
// }

// fn transpile_value_expr(
//     expr: &syn::Expr,
//     ctx: &mut TranspileCtx,
// ) -> Result<LuaExpression, TranspileError> {
//     match expr {
//         syn::Expr::Array(expr_array) => expr_array
//             .transpile(ctx)
//             .map(|e| LuaExpression::TableConstructor(Box::new(e))),
//         syn::Expr::Assign(expr_assign) => expr_assign.transpile_iife(ctx),
//         syn::Expr::Async(expr_async) => unsupported_rust_feat!("async keyword"),
//         syn::Expr::Await(expr_await) => unsupported_rust_feat!("await keyword"),
//         syn::Expr::Binary(expr_binary) => todo!(),
//         syn::Expr::Block(expr_block) => todo!(),
//         syn::Expr::Break(expr_break) => unsupported_rust_feat!("break (value expression)"),
//         syn::Expr::Call(expr_call) => todo!(),
//         syn::Expr::Cast(expr_cast) => unsupported_rust_feat!("cast expression"),
//         syn::Expr::Closure(expr_closure) => todo!(),
//         syn::Expr::Const(expr_const) => todo!(),
//         syn::Expr::Continue(expr_continue) => unsupported_rust_feat!("continue (value expression)"),
//         syn::Expr::Field(expr_field) => todo!(),
//         syn::Expr::ForLoop(expr_for_loop) => todo!(),
//         syn::Expr::Group(expr_group) => todo!(),
//         syn::Expr::If(expr_if) => todo!(),
//         syn::Expr::Index(expr_index) => todo!(),
//         syn::Expr::Infer(expr_infer) => todo!(),
//         syn::Expr::Let(expr_let) => todo!(),
//         syn::Expr::Lit(expr_lit) => Ok(expr_lit.transpile(ctx)?),
//         syn::Expr::Loop(expr_loop) => todo!(),
//         syn::Expr::Macro(expr_macro) => todo!(),
//         syn::Expr::Match(expr_match) => todo!(),
//         syn::Expr::MethodCall(expr_method_call) => todo!(),
//         syn::Expr::Paren(expr_paren) => todo!(),
//         syn::Expr::Path(expr_path) => todo!(),
//         syn::Expr::Range(expr_range) => todo!(),
//         syn::Expr::RawAddr(expr_raw_addr) => unsupported_rust_feat!("raw address"),
//         syn::Expr::Reference(expr_reference) => todo!(),
//         syn::Expr::Repeat(expr_repeat) => todo!(),
//         syn::Expr::Return(expr_return) => todo!(),
//         syn::Expr::Struct(expr_struct) => todo!(),
//         syn::Expr::Try(expr_try) => unsupported_rust_feat!("? operator"),
//         syn::Expr::TryBlock(expr_try_block) => unsupported_rust_feat!("try block keyword"),
//         syn::Expr::Tuple(expr_tuple) => todo!(),
//         syn::Expr::Unary(expr_unary) => todo!(),
//         syn::Expr::Unsafe(expr_unsafe) => unsupported_rust_feat!("unsafe"),
//         syn::Expr::Verbatim(token_stream) => unsupported_rust_feat!("verbatim"),
//         syn::Expr::While(expr_while) => todo!(),
//         syn::Expr::Yield(expr_yield) => unsupported_rust_feat!("yield"),
//         _ => unsupported_rust_feat!("[unknown] expression"),
//     }
// }

// impl Transpile for syn::ExprLit {
//     type Output = LuaExpression;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         match &self.lit {
//             syn::Lit::Str(lit_str) => todo!(),
//             syn::Lit::ByteStr(lit_byte_str) => todo!(),
//             syn::Lit::CStr(lit_cstr) => todo!(),
//             syn::Lit::Byte(lit_byte) => todo!(),
//             syn::Lit::Char(lit_char) => todo!(),
//             syn::Lit::Int(lit_int) => Ok(lit_int.transpile(ctx)?),
//             syn::Lit::Float(lit_float) => todo!(),
//             syn::Lit::Bool(lit_bool) => todo!(),
//             syn::Lit::Verbatim(literal) => todo!(),
//             _ => unsupported_rust_feat!("unknown literal"),
//         }
//     }
// }

// impl Transpile for syn::LitInt {
//     type Output = LuaExpression;

//     fn transpile(&self, ctx: &mut TranspileCtx) -> Result<Self::Output, TranspileError> {
//         Ok(LuaExpression::Number(TokenReference::new(
//             vec![],
//             Token::new(TokenType::Number {
//                 text: ShortString::new(self.token().to_string()),
//             }),
//             vec![],
//         )))
//     }
// }
