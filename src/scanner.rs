use crate::error::*;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, CfgError> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {}
                Err(e) => {
                    e.report("".to_string());
                    std::process::exit(11);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Result<(), CfgError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::SemiColon),
            '/' => {
                if self.is_match('/') {
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.is_match('*') {
                    // block comment
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '!' => {
                let tok = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.is_match('=') {
                    TokenType::Equal
                } else {
                    TokenType::Assign
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            '0'..='9' => self.number()?,

            _ if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => {
                return Err(CfgError::error(
                    self.line,
                    "Unexpected character".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), CfgError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.is_match('/') {
                        return Ok(());
                    }
                }

                Some('/') => {
                    self.advance();
                    if self.is_match('*') {
                        self.scan_comment()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(CfgError::error(
                        self.line,
                        "unterminate comment".to_string(),
                    ));
                }

                _ => {
                    self.advance();
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(ttype) = Scanner::keyword(text.as_str()) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Indentifier);
        }
    }

    fn number(&mut self) -> Result<(), CfgError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
        Ok(())
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Result<(), CfgError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(CfgError::error(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }
        self.advance();
        //TODO
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "class" => Some(TokenType::Class),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "var" => Some(TokenType::Var),
            "nil" => Some(TokenType::Nil),
            "if" => Some(TokenType::If),
            "this" => Some(TokenType::This),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "print" => Some(TokenType::Print),
            _ => None,
        }
    }

    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None)
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexname: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexname, literal, self.line));
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }
}
