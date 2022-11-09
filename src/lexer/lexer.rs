use crate::loxr_::Loxr;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Token_Type {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub fn keywords(k: &str) -> Option<Token_Type> {
    match k {
        "and" => Some(Token_Type::AND),
        "class" => Some(Token_Type::CLASS),
        "else" => Some(Token_Type::ELSE),
        "false" => Some(Token_Type::FALSE),
        "for" => Some(Token_Type::FOR),
        "fun" => Some(Token_Type::FUN),
        "if" => Some(Token_Type::IF),
        "nil" => Some(Token_Type::NIL),
        "or" => Some(Token_Type::OR),
        "print" => Some(Token_Type::PRINT),
        "return" => Some(Token_Type::RETURN),
        "super" => Some(Token_Type::SUPER),
        "this" => Some(Token_Type::THIS),
        "true" => Some(Token_Type::TRUE),
        "var" => Some(Token_Type::VAR),
        "while" => Some(Token_Type::WHILE),
        _ => None,
    }
}
#[derive(Debug)]
pub enum Literal {
    STRING(String),
    NUM(f32),
    NONE(char),
}

#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: Token_Type,
    lexeme: &'a str,
    pub literal: Option<Literal>,
    line: usize,
}
#[allow(dead_code)]
pub struct Scanner<'a> {
    pub tokens: Vec<Token<'a>>,
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    source_as_vec: Vec<char>,
    loxr: &'a mut Loxr,
}

#[allow(dead_code)]
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, loxr: &'a mut Loxr) -> Scanner<'a> {
        return Scanner {
            tokens: Vec::<Token>::new(),
            source,
            start: 0,
            current: 0,
            line: 1,
            source_as_vec: source.chars().collect(),
            loxr,
        };
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token_Type::EOF, None);
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(Token_Type::LEFT_PAREN, None),
            ')' => self.add_token(Token_Type::RIGHT_PAREN, None),
            '{' => self.add_token(Token_Type::LEFT_BRACE, None),
            '}' => self.add_token(Token_Type::RIGHT_BRACE, None),
            ',' => self.add_token(Token_Type::COMMA, None),
            '.' => self.add_token(Token_Type::DOT, None),
            '+' => self.add_token(Token_Type::PLUS, None),
            ';' => self.add_token(Token_Type::SEMICOLON, None),
            '*' => self.add_token(Token_Type::STAR, None),

            '!' => self.match_next_add_token('=', Token_Type::BANG_EQUAL, Token_Type::BANG),
            '>' => self.match_next_add_token('=', Token_Type::GREATER_EQUAL, Token_Type::GREATER),
            '<' => self.match_next_add_token('=', Token_Type::LESS_EQUAL, Token_Type::LESS),
            '=' => self.match_next_add_token('=', Token_Type::EQUAL_EQUAL, Token_Type::EQUAL),

            '/' => {
                if self.source_as_vec[self.current] == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } 
                else if self.peek() == '*' {
                    self.advance();
                    loop {
                        if self.is_at_end() {
                            self.loxr.error(self.line,String::from("No ending to block comment"));
                        }
                        if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance();
                            self.advance();
                            break;
                        }
                        else {
                            self.advance();
                        }
                    }
                } 
                else {
                    self.add_token(Token_Type::SLASH, None);
                }
            }

            '\n' => self.line = self.line + 1,

            ' ' => {}
            '\r' => {}
            '\t' => {}

            '"' => self.string(),

            other => {
                if other.is_numeric() {
                    self.number();
                } else if other.is_alphanumeric() {
                    self.identifier();
                }
                self.loxr.error(self.line, String::from("Invalid token"))
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        self.add_token(keywords(lexeme).unwrap_or(Token_Type::IDENTIFIER),None);
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }
        println!("while loop ends");
        println!("{} {}", self.peek(), self.peek_next());
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let num: f32 = self.source[self.start..self.current]
            .parse()
            .unwrap_or_else(|_| {
                self.loxr
                    .error(self.line, String::from("Cannot parse numeric literal"));
                0.1
            });

        self.add_token(Token_Type::NUMBER, Some(Literal::NUM(num)));
    }

    fn match_next_add_token(
        &mut self,
        expected: char,
        matches: Token_Type,
        matches_not: Token_Type,
    ) {
        if self.is_at_end() || self.source_as_vec[self.current] != expected {
            self.add_token(matches_not, None);
        } else {
            self.current = self.current + 1;
            self.add_token(matches, None);
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source_as_vec[self.current + 1];
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1
            };
            self.advance();
        }

        if self.is_at_end() {
            self.loxr
                .error(self.line, String::from("Unterminated string"));
            return;
        };

        self.advance();
        let str_val = &self.source[self.start + 1..self.current - 1];
        self.add_token(
            Token_Type::STRING,
            Some(Literal::STRING(str_val.to_string())),
        );
    }

    fn advance(&mut self) -> char {
        let current_char = self.source_as_vec[self.current];
        self.current = self.current + 1;
        return current_char;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        return self.source_as_vec[self.current];
    }

    fn add_token(&mut self, token_type: Token_Type, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: literal,
            line: self.line,
        })
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn create_token(token_type: Token_Type, lexeme: &str) -> Token {
        return Token {
            token_type,
            lexeme: lexeme,
            literal: None,
            line: 1,
        };
    }
    #[test]
    fn test_single_character_tokens() {
        let mut loxr = Loxr::new();

        // let mut scanner = Scanner::new("{}.()**}}", &mut loxr);
        let mut scanner = Scanner::new("{}.()**}}!===!>=><=>", &mut loxr);
        let mut expected = Vec::<Token>::new();

        expected.push(create_token(Token_Type::LEFT_BRACE, "{"));
        expected.push(create_token(Token_Type::RIGHT_BRACE, "}"));
        expected.push(create_token(Token_Type::DOT, "."));
        expected.push(create_token(Token_Type::LEFT_PAREN, "("));
        expected.push(create_token(Token_Type::RIGHT_PAREN, ")"));
        expected.push(create_token(Token_Type::STAR, "*"));
        expected.push(create_token(Token_Type::STAR, "*"));
        expected.push(create_token(Token_Type::RIGHT_BRACE, "}"));
        expected.push(create_token(Token_Type::RIGHT_BRACE, "}"));
        expected.push(create_token(Token_Type::BANG_EQUAL, "!="));
        expected.push(create_token(Token_Type::EQUAL_EQUAL, "=="));
        expected.push(create_token(Token_Type::BANG, "!"));
        expected.push(create_token(Token_Type::GREATER_EQUAL, ">="));
        expected.push(create_token(Token_Type::GREATER, ">"));
        expected.push(create_token(Token_Type::LESS_EQUAL, "<="));
        expected.push(create_token(Token_Type::GREATER, ">"));
        expected.push(create_token(Token_Type::EOF, ""));

        let matching = scanner
            .scan_tokens()
            .iter()
            .zip(&expected)
            .filter(|&(a, b)| a.token_type == b.token_type)
            .count();

        println!("{} {}", matching, scanner.tokens.len());
        assert_eq!(matching == scanner.tokens.len(), true);
        assert_eq!(scanner.loxr.had_error, false);
    }

    #[test]
    fn should_return_err() {
        let mut loxr = Loxr::new();
        let mut scanner = Scanner::new("{}dfg{}", &mut loxr);
        scanner.scan_tokens();
        assert_eq!(scanner.loxr.had_error, true);
    }
}
