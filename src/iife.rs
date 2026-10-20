use crate::transpiler::Transpile;
use crate::transpiler::TranspileCtx;
use crate::transpiler::TranspileError;
use full_moon::{
    ast::{
        AnonymousFunction as LuaAnonymousFunction, Block as LuaBlock, Expression as LuaExpression,
        FunctionBody as LuaFunctionBody, FunctionCall as LuaFunctionCall, LastStmt as LuaLastStmt,
        Prefix as LuaPrefix, Return as LuaReturn, Stmt as LuaStmt,
        punctuated::{Pair as LuaPair, Punctuated as LuaPunctuated},
        span::ContainedSpan as LuaContainedSpan,
    },
    tokenizer::TokenReference as LuaTokenReference,
};

impl TranspileIIFE for syn::ExprAssign {
    fn transpile_iife(&self, ctx: &mut TranspileCtx) -> Result<LuaExpression, TranspileError> {
        let nil = LuaExpression::Symbol(
            LuaTokenReference::symbol("nil").expect("nil is a valid Lua symbol"),
        );

        let mut returns = LuaPunctuated::new();
        returns.push(LuaPair::new(nil, None));

        let lua_return = LuaReturn::new().with_returns(returns);

        // let block = LuaBlock::new()
        //     .with_stmts(vec![
        //         self.transpile(ctx)
        //             .map(|s| (LuaStmt::Assignment(s), None))?,
        //     ])
        //     .with_last_stmt(Some((LuaLastStmt::Return(lua_return), None)));

        todo!()

        // Ok(IIFE::wrap(block))
    }
}

pub trait TranspileIIFE {
    // Specifically should return a function call that invokes a parenthesis-wrapped anonymous function
    fn transpile_iife(&self, ctx: &mut TranspileCtx) -> Result<LuaExpression, TranspileError>;
}

pub struct IIFE;

impl IIFE {
    pub fn wrap(block: LuaBlock) -> LuaExpression {
        LuaExpression::FunctionCall(LuaFunctionCall::new(LuaPrefix::Expression(Box::new(
            LuaExpression::Parentheses {
                contained: LuaContainedSpan::new(
                    LuaTokenReference::symbol("(").expect("valid opening parenthesis"),
                    LuaTokenReference::symbol(")").expect("valid closing parenthesis"),
                ),
                expression: Box::new(LuaExpression::Function(Box::new(
                    LuaAnonymousFunction::new().with_body(LuaFunctionBody::new().with_block(block)),
                ))),
            },
        ))))
    }
}
