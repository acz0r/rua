use std::{fmt, fs};

use full_moon::{
    ast::{
        Ast, Call, Expression, FunctionArgs, FunctionCall, Prefix, Suffix,
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
    },
    tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
};
use stylua_lib::{OutputVerification, QuoteStyle};
use thiserror::Error;

use crate::mappings::ident;

pub mod mappings;
pub mod test;

fn main() {
    let test_file = fs::read_to_string("src/test.rs").unwrap();
    let ast = syn::parse_file(&test_file).unwrap();

    let style_cfg = stylua_lib::Config {
        column_width: usize::MAX,
        quote_style: QuoteStyle::ForceDouble,
        ..Default::default()
    };
    let lua_out = stylua_lib::format_ast(
        transpile(&ast, &TranspileOptions::default()).unwrap(),
        style_cfg,
        None,
        OutputVerification::None,
    )
    .unwrap();

    fs::write("src/test.lua", lua_out.to_string()).unwrap();
}

/// Transpiles a Rust AST into a Full Moon AST (Rust -> Lua).
pub fn transpile(rust_file: &syn::File, opts: &TranspileOptions) -> Result<Ast, TranspileError> {
    let mut u_ctx = UniversalTranspileCtx::default();
    rust_file.transpile_default_ctx(&mut u_ctx, opts)
}

pub fn token_ref(token_ty: TokenType) -> TokenReference {
    TokenReference::new(vec![], Token::new(token_ty), vec![])
}

pub fn ident(s: impl AsRef<str>) -> TokenReference {
    token_ref(TokenType::Identifier {
        identifier: s.as_ref().into(),
    })
}

pub fn symbol(s: Symbol) -> TokenReference {
    token_ref(TokenType::Symbol { symbol: s })
}

pub fn num_lit(s: impl AsRef<str>) -> Expression {
    Expression::Number(token_ref(TokenType::Number {
        text: s.as_ref().into(),
    }))
}

pub fn bool_lit(value: bool) -> Expression {
    Expression::Symbol(symbol(if value { Symbol::True } else { Symbol::False }))
}

pub fn comma_separated<T>(items: Vec<T>) -> Punctuated<T> {
    let mut punctuated = Punctuated::new();
    let len = items.len();
    for (i, item) in items.into_iter().enumerate() {
        if i < len - 1 {
            punctuated.push_punctuated(item, symbol(Symbol::Comma));
        } else {
            punctuated.push(Pair::End(item));
        }
    }
    punctuated
}

pub fn fn_call(ident: TokenReference, args: Vec<Expression>) -> Expression {
    Expression::FunctionCall(FunctionCall::new(Prefix::Name(ident)).with_suffixes(vec![
        Suffix::Call(Call::AnonymousCall(Box::new(FunctionArgs::Parentheses {
            parentheses: ContainedSpan::new(symbol(Symbol::LeftParen), symbol(Symbol::RightParen)),
            arguments: comma_separated(args),
        }))),
    ]))
}

pub fn str_lit(s: impl AsRef<str>) -> Expression {
    Expression::String(token_ref(TokenType::StringLiteral {
        literal: s.as_ref().into(),
        multi_line_depth: 0,
        quote_type: StringLiteralQuoteType::Double,
    }))
}

pub fn single<T>(item: T) -> Punctuated<T> {
    comma_separated(vec![item])
}

pub trait Transpile {
    type Ctx;
    type Output;

    fn transpile(
        &self,
        ctx: &mut Self::Ctx,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError>;
    fn transpile_default_ctx(
        &self,
        u_ctx: &mut UniversalTranspileCtx,
        opts: &TranspileOptions,
    ) -> Result<Self::Output, TranspileError>
    where
        Self::Ctx: Default,
    {
        self.transpile(&mut Self::Ctx::default(), u_ctx, opts)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TranspileOptions;

#[derive(Debug, Clone, Error)]
pub enum TranspileError {
    #[error(transparent)]
    Unsupported(Unsupported),
}

#[derive(Debug, Clone)]
pub struct Unsupported {
    pub item: &'static str,
    pub details: Option<&'static str>,
}

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported rust item `{}`", self.item)?;
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for Unsupported {}

#[macro_export]
macro_rules! unsupported {
    ($item:expr) => {
        return Err(crate::TranspileError::Unsupported(crate::Unsupported {
            item: $item,
            details: None,
        }))
    };
    ($item:expr, $details:expr) => {
        return Err(crate::TranspileError::Unsupported(crate::Unsupported {
            item: $item,
            details: Some($details),
        }))
    };
}

#[derive(Debug, Clone, Default)]
pub struct UniversalTranspileCtx {
    pub counter: usize,
}

impl UniversalTranspileCtx {
    pub fn unique_var(&mut self) -> TokenReference {
        // Implement a method to generate a unique variable name.
        // This is a placeholder implementation; you should replace it with your actual logic.
        let name = format!("_tmp{}", self.counter);
        self.counter += 1;
        ident(name)
    }
}
