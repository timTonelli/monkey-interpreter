use std::{fmt::Display, ops::Deref};

/*
* Abstract Syntax Tree
*/

/*
* TODO: Explore the idea of generalizing this data structure with generics
* Ast, Block, Arguments and Parameters are all very similar implementations
*/
pub struct Ast(Vec<Statement>);

impl From<Vec<Statement>> for Ast {
    fn from(value: Vec<Statement>) -> Self {
        Ast(value)
    }
}

impl Deref for Ast {
    type Target = Vec<Statement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
* Statements
*/
#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: Indentifier,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let { name, value } => write!(f, "let {} = {};", name, value),
            Self::Return(expr) => write!(f, "return {};", expr),
            Self::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = &self
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        write!(f, "{}", string)
    }
}

/*
* Expressions
*/
#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(Indentifier),
    IntLiteral(i64),
    BooleanLiteral(bool),
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Block,
        alternative: Option<Block>,
    },
    FuncLiteral {
        parameters: Parameters,
        body: Block,
    },
    Call {
        function: Box<Expression>,
        arguments: Arguments,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(i) => write!(f, "{}", i),
            Self::IntLiteral(i) => write!(f, "{}", i),
            Self::BooleanLiteral(b) => write!(f, "{}", b),
            Self::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Self::Infix {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Self::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, " else {}", alt)?;
                }
                Ok(())
            }
            Self::FuncLiteral { parameters, body } => {
                write!(f, "fn({}) {{ {} }}", parameters, body)
            }
            Self::Call {
                function,
                arguments,
            } => {
                write!(f, "{}({})", function, arguments)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Indentifier(String);

impl Display for Indentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Indentifier {
    fn from(value: String) -> Self {
        Indentifier(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameters(pub Vec<Expression>);

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq)]
pub struct Arguments(pub Vec<Expression>);

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", string)
    }
}

/*
* Operators
*/
#[derive(Debug, PartialEq)]
pub enum Operator {
    Bang,
    Plus,
    Minus,
    Multiplication,
    Division,
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<"),
            Self::Equals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
        }
    }
}
