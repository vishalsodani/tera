use std::collections::HashMap;
use std::fmt;

/// All math operators
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MathOperator {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
}

impl fmt::Display for MathOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            MathOperator::Add => "+",
            MathOperator::Sub => "-",
            MathOperator::Mul => "*",
            MathOperator::Div => "/",
        })
    }
}

/// All logic operators
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicOperator {
    /// >
    Gt,
    /// >=
    Gte,
    /// <
    Lt,
    /// <=
    Lte,
    /// ==
    Eq,
    /// !=
    NotEq,
    /// and
    And,
    /// or
    Or,
}

impl fmt::Display for LogicOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            LogicOperator::Gt => ">",
            LogicOperator::Gte => ">=",
            LogicOperator::Lt => "<",
            LogicOperator::Lte => "<=",
            LogicOperator::Eq => "==",
            LogicOperator::NotEq => "!=",
            LogicOperator::And => "and",
            LogicOperator::Or => "or",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

/// A struct that contains the position of the node
#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

/// All literals used in templates by template writers
///
/// eg 42, "hello", true, 0.1
#[derive(Clone, Debug, PartialEq)]
pub enum LitKind {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}
pub type Lit = Spanned<LitKind>;

/// A filter node `| round(method="ceil")`
#[derive(Clone, Debug, PartialEq)]
pub struct Filter {
    name: String,
    params: HashMap<String, Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    name: String,
    filters: Vec<Filter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    operator: MathOperator,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogicExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    operator: LogicOperator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Ident(Ident),
    Lit(Lit),
    Math(MathExpr),
    Logic(LogicExpr),
    //Test(Test),
}
pub type Expr = Spanned<ExprKind>;

/// A filter section node `{{ filter name(param="value") }} content {{ endfilter }}`
#[derive(Clone, Debug, PartialEq)]
pub struct FilterSection {
    name: String,
    params: HashMap<String, Expr>,
    body: Vec<Node>,
}

/// Set a variable in the context `{% set val = "hey" %}`
#[derive(Clone, Debug, PartialEq)]
pub struct Set {
    ident: String,
    value: Expr,
}

/// A test node `if my_var is odd`
#[derive(Clone, Debug, PartialEq)]
pub struct Test {
    /// Which expression is evaluated
    expr: Expr,
    /// Name of the test
    name: String,
    /// Any optional param given to the test
    params: Vec<Expr>
}

/// All Tera nodes that can be encountered
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// A call to `{{ super() }}` in a block
    Super,
    /// Some actual text
    Text(String),
    /// The text between `{% raw %}` and `{% endraw %}`
    Raw(String),
    /// The `{% extends "blabla.html" %}` node, contains the template name
    Extends(String),
    /// The `{% include "blabla.html" %}` node, contains the template name
    Include(String),
    /// The full template AST
    Template(Vec<Node>),
}
