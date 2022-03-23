use crate::token_type::*;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        match self {
           Object::Num(x) => write!(f, "{x}"),
           Object::Str(x) => write!(f, "\"{x}\""),
           Object::Nil => write!(f, "nil"),
           Object::True => write!(f, "true"),
           Object::False => write!(f, "false")
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexname: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexname: String, literal: Option<Object>, line: usize)-> Token{
        Token{ttype, lexname, literal, line}
    }

    pub fn eof(line: usize) ->Token {
        Token{
            ttype: TokenType::Eof, 
            lexname:"".to_string(), 
            literal: None, 
            line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        write!(f, "{:?} {} {}", 
        self.ttype, self.lexname, if let Some(literal) = &self.literal {
            literal.to_string()
        } else {
            "None".to_string()
        })
    }
}

/*
pub enum Token {
    literal :{lexname :String, literal : <...>},
    Keyword : {lexname: String, ttype:String},
}
*/