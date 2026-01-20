#![allow(dead_code)]

use std::time::Instant;

use super::ast::*;
use crate::lexer::{diagnostic::Diagnostic, token::Token};

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
    #[inline]
    fn advance(&mut self) -> Option<&Token<'src>> {
        let tok: &Token<'_> = self.cur()?;
        self.pos += 1;
        Some(tok)
    }

    pub fn parse(&mut self, debug: bool) -> Result<Vec<Stmt<'src>>, Vec<Diagnostic<'t, 'src>>> {
        let mut nodes: Vec<Stmt<'src>> = vec![];
        let start: Instant = Instant::now();

        while let Some(cur) = self.cur() {
            match cur {
                // control flow
                Token::Break => nodes.push(Stmt::Break),
                Token::Continue => nodes.push(Stmt::Continue),

                // TODO: wire this to the SyntaxError setup i alr have
                _ => if debug {println!("not implemented: {cur}")},
            }
            if debug {println!("Parsed: {cur}")};
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