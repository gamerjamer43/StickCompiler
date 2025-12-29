// the lexer itself, the big beef
use super::error::{LexError, err};
use logos::{Logos, skip};

// the entire token spec. this also doubles as the lexer itself pretty much, as we j
#[derive(Logos, Default, Debug, PartialEq)]
#[logos(error(LexError<'s>, err))] // TODO: fully understand why 's shuts this up, and why i can't use '_ or 'src
#[logos(skip r"[ \t\n\f\r]+")] // ignore tabs, newlines, form feeds, and carriage returns
pub enum Token<'src> {
    // reuse this if needed
    // #[token(r#"match")]
    // Keyword,

    // comments (skipped)
    // TODO: figure out how to avoid greedy comments
    #[regex(r"//[^\r\n]*", skip, allow_greedy = true)]
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", skip)]
    Comment,

    // equality and comparisons
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<=")]
    LessEq,
    #[token(">=")]
    GreaterEq,

    // shifts and compound assigns
    #[token("<<=")]
    ShlEq,
    #[token(">>=")]
    ShrEq,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,

    // arithmetic compound assigns
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token("%=")]
    PercentEq,

    // bitwise compound assigns
    #[token("&=")]
    AndEq,
    #[token("|=")]
    OrEq,
    #[token("^=")]
    XorEq,

    // punctuation and others (member access and shit)
    #[token("::")]
    DoubleColon,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("|->")]
    Match,

    // ranges and varargs style (may not use)
    #[token("...")]
    Elipses,
    #[token("..")]
    DotDot,

    // arithmetic
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    // assignment
    #[token("=")]
    Assign,

    // comparisons
    #[token("<")]
    Less,
    #[token(">")]
    Greater,

    // bitwise
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("^")]
    BitXor,
    #[token("~")]
    BitNot,

    // logical ops
    #[token("not")]
    LogicalNot,
    #[token("and")]
    LogicalAnd,
    #[token("or")]
    LogicalOr,

    // punctuation
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("?")]
    Question,

    // grouping / separators
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,

    /// true or false is properly handled at parse time as a boolean. borrowed from https://logos.maciej.codes/examples/json.html
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    /// identifiers cannot start with a number, and can only contain A-Z, a-z, 0-9, and _
    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,

    /// strings are anything enclosed inside "", no closure == borrow slices of the string at parse time, avoids heap allocating a copy
    // #[regex(r#""([^"\\\n]|\\.)*""#, |lex| lex.slice().to_owned())]
    #[regex(r#""([^"\\\n]|\\.)*""#)]
    LitString,

    /// chars are single characters (including escaped chars) enclosed in ''
    #[regex(r#"'([^'\\\n]|\\.)'"#)]
    LitChar,

    #[regex(r"[0-9]+(?:_[0-9]+)*\.[0-9]+", |lex| lex.slice())]
    LitFloat(&'src str),

    #[regex(r"[0-9]+(?:_[0-9]+)*", |lex| lex.slice())]
    LitInteger(&'src str),

    // this will throw a lexerror
    #[default]
    UnsupportedToken,
}
