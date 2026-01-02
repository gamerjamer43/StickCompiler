#![allow(dead_code)]

/// all operators featured in the language set
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // binary
    Add, Sub, Mul, Div, Mod,

    // equality operations
    Eq, NotEq, Less, LessEq, Greater, GreaterEq,

    // logical operations
    And, Or, Not,

    // bitwise
    BitAnd, BitOr, BitNot, BitXor, Shl, Shr,

    // assignment
    Assign,
    AddAssign, SubAssign, MulAssign, DivAssign, ModAssign,
    AndAssign, OrAssign, XorAssign, ShlAssign, ShrAssign,
}

/// general expressions which will be recursively parsed using chumsky
/// box anything recursive, as otherwise the enum will be infinite, and rust needs
/// to know the size at compile time.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'src> {
    // literals
    Int(&'src str),
    Float(&'src str),
    Bool(bool),
    String(&'src str),
    Char(&'src str),
    Ident(&'src str),

    // unary operations
    Neg(Box<Expr<'src>>),
    Not(Box<Expr<'src>>),
    BitNot(Box<Expr<'src>>),

    // binary ops
    BinOp {
        op: Operator,
        lhs: Box<Expr<'src>>,
        rhs: Box<Expr<'src>>
    },

    // function call
    Call {
        func: Box<Expr<'src>>,
        args: Vec<Expr<'src>>
    },

    // field access (a.b)
    Field {
        obj: Box<Expr<'src>>,
        name: &'src str,
    },

    // index access (a[b])
    Index {
        obj: Box<Expr<'src>>,
        index: Box<Expr<'src>>,
    },

    // control flow
    If {
        cond: Box<Expr<'src>>,
        then: Box<Expr<'src>>,
        else_: Option<Box<Expr<'src>>>,
    },

    While {
        cond: Box<Expr<'src>>,
        body: Box<Expr<'src>>,
    },

    For {
        name: &'src str,
        iter: Box<Expr<'src>>,
        body: Box<Expr<'src>>,
    },

    Block(Vec<Stmt<'src>>),
}

/// all types of statement. either control or a normal expression
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'src> {
    Expr(Expr<'src>),
    Return(Option<Expr<'src>>),
    Break,
    Continue,
}