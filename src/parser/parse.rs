#![allow(dead_code, unused_variables)]

use std::time::Instant;

use super::ast::*;
use crate::lexer::{diagnostic::Diagnostic, token::Token};

// AGHHHHHHHHHHHHH i did operations stupidly so now imma have to debloat later
// this might be the most ugly thing you see in here i apologize
// so let me put it up on display. SHAME ME!
impl<'src> Token<'src> {
    pub fn as_assign_op(&self) -> Option<AssignOp> {
        use AssignOp::*;
        match self {
            Token::Assign    => Some(Assign),
            Token::PlusEq    => Some(PlusEq),
            Token::MinusEq   => Some(MinusEq),
            Token::StarEq    => Some(StarEq),
            Token::SlashEq   => Some(SlashEq),
            Token::PercentEq => Some(PercentEq),
            Token::AndEq     => Some(AndEq),
            Token::OrEq      => Some(OrEq),
            Token::XorEq     => Some(XorEq),
            Token::ShlEq     => Some(ShlEq),
            Token::ShrEq     => Some(ShrEq),
            _ => None,
        }
    }
}

// didn't tie parser lifetime to source
pub struct Parser<'src, 't> {
    pub path: &'src str,
    pub src: &'src str,
    pub tokens: &'t [Token<'src>],
    pub pos: usize,
}

impl<'src, 't> Parser<'src, 't> {
    #[inline]
    fn cur(&self) -> Option<&'t Token<'src>> {
        self.tokens.get(self.pos)
    }

    #[inline]
    fn peek(&self) -> Option<&'t Token<'src>> {
        self.tokens.get(self.pos + 1)
    }

    #[inline]
    fn expect<F>(&mut self, f: F) -> Option<&Token<'src>>
    where
        F: FnOnce(&Token<'_>) -> bool,
    {
        let tok = self.cur()?;
        if f(tok) {
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    // leaving some shit intentionally _ because i'll do lifetimes later
    #[inline] fn advance(&mut self) -> Option<&Token<'src>> { self.advance_by(1) }

    #[inline]
    fn advance_by(&mut self, n: u8) -> Option<&Token<'src>> {
        let tok: &Token<'src> = self.cur()?;
        self.pos += n as usize;
        Some(tok)
    }

    #[inline]
    fn parse_prefix(&mut self) -> Expr<'src> {
        // TODO: write proper error handling... and parse expr... and test this
        let tok: &Token<'_> = self.advance().expect("unexpected EOF");
        match tok {
            // Token::Minus       => Expr::Unary { op: UnaryOp::Neg, expr: Box::new(self.parse_expr(12)) },
            // Token::LogicalNot  => Expr::Unary { op: UnaryOp::Not, expr: Box::new(self.parse_expr(12)) },
            // Token::BitNot      => Expr::Unary { op: UnaryOp::BitNot, expr: Box::new(self.parse_expr(12)) },

            Token::Identifier(name) => Expr::Ident(name),
            Token::LitInteger(n)    => Expr::Literal(Literal::Int(n)),
            Token::LitFloat(n)      => Expr::Literal(Literal::Float(n)),
            Token::LitString(s)     => Expr::Literal(Literal::String(s)),
            Token::LitChar(c)       => Expr::Literal(Literal::Char(c)),
            Token::Bool(b)          => Expr::Literal(Literal::Bool(*b)),

            // Token::LParen => {
            //     let e = self.parse_expr(0);
            //     self.expect(|t| matches!(t, Token::RParen)).expect("missing ')'");
            //     e
            // }

            // Token::If => self.parse_if_expr(),
            // Token::While => self.parse_while_expr(),
            // Token::Match => self.parse_match_expr(),
            // Token::LBrace => self.parse_block_expr(),

            _ => todo!("not finished parsing. error @ line 100 ./parser/parse.rs: {tok:?}"),
        }
    }

    // determine if an ident is a decl/assmt, reassmt, or just a plain ident
    // TODO add parse_expr
    #[inline]
    fn parse_ident(&mut self) -> Stmt<'src> {
        // unused frn
        let name: &str = match self.cur() {
            Some(Token::Identifier(name)) => name,
            _ => unreachable!("parse_ident called when cur token isn't Identifier"),
        };

        match self.peek() {
            // AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
            Some(op @ (Token::Assign
                | Token::PlusEq | Token::MinusEq | Token::StarEq 
                | Token::SlashEq | Token::PercentEq
                | Token::AndEq | Token::OrEq | Token::XorEq
                | Token::ShlEq | Token::ShrEq)
            ) => {
                self.advance_by(2);
                match op.as_assign_op() {
                    Some(parsed_op) => { 
                        // let rhs = self.parse_expr();
                        // Stmt::Expr(Expr::Assign { op: parsed_op, lhs: LeftSide::Var(Ident(name)), rhs: ? });
                        todo!("ain't done with the properly parsed op case case on parsed_op yet. ~line 135, ./parser/parse.rs");
                    }
                    None => panic!("havent setup error handling. as_assign_op() returned None. ~line 135, ./parser/parse.rs"),
                }
            }

            // nothing matched
            Some(_) => {
                panic!("expected '=' or ';' after identifier");
                // self.advance();
                // Stmt::Error
            }

            // last char is an ident
            None => {
                panic!("unexpected end of input after identifier (expected ';' or assignment)");
                // self.advance();
                // Stmt::Error
            }
        }
    }

    pub fn parse(&mut self, debug: bool) -> Result<Vec<Stmt<'src>>, Vec<Diagnostic<'t, 'src>>> {
        let mut nodes: Vec<Stmt<'src>> = Vec::new();
        let start: Instant = Instant::now();
        if debug { println!(); }

        while let Some(cur) = self.cur() {
            match cur {
                // idents (read parse_ident)
                Token::Identifier(_) => nodes.push(self.parse_ident()),

                // control flow
                Token::Break => nodes.push(Stmt::Break),
                Token::Continue => nodes.push(Stmt::Continue),

                // TODO: wire this to the SyntaxError setup i alr have
                _ => {
                    if debug { println!("not implemented: {cur}"); }
                    self.advance();

                    // i should never use this
                    continue;
                }
            }

            if debug {
                println!("Parsed: {cur}");
            }
            self.advance();
        }

        println!(
            "Parsed {} tokens into {} nodes. Took {}s.",
            self.tokens.len(),
            nodes.len(),
            start.elapsed().as_secs_f64()
        );
        Ok(nodes)
    }
}