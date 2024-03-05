use token::{Token, TokenType};

pub struct Lexer {
  code: Vec<char>,
  cursor: usize,
}

impl Lexer {
  pub fn new(code: String) -> Self {
    Self {
      cursor: 0,
      code: code.chars().collect(),
    }
  }

  pub fn tokenized(&mut self) -> (Option<String>, Vec<Token>) {
    let mut tokens: Vec<Token> = Vec::new();

    loop {
      if let Some(cur) = self.get_cur() {
        match cur {
          _ if cur.is_whitespace() => {}, 
          '+' => {
            tokens.push(Token { 
              token_type: TokenType::Plus,
              token_value: None,
            });
          },
          '-' => {
            tokens.push(Token { 
              token_type: TokenType::Minus,
              token_value: None,
            });
          },
          '/' => {
            tokens.push(Token { 
              token_type: TokenType::Div,
              token_value: None,
            });
          },
          '*' => {
            tokens.push(Token { 
              token_type: TokenType::Star,
              token_value: None,
            });
          },
          '(' => {
            tokens.push(Token { 
              token_type: TokenType::OpenParen,
              token_value: None,
            });
          },
          ')' => {
            tokens.push(Token { 
              token_type: TokenType::CloseParen,
              token_value: None,
            });
          },
          _ if cur.is_numeric() => {
            let (err, num, dot_count) = self.make_number();
            if err != None {
              return (err, tokens); 
            }
            tokens.push(Token {
              token_type: if dot_count > 0 { TokenType::Float } else { TokenType::Integer },
              token_value: Some(num),
            });
          },
          all => {
            return (Some(format!("Invalid character found `{}`", all)), tokens);
          },
        }

        self.advance();
      } else {
        break;
      }
    }
    // tokens.push(Token {
    //   token_type: TokenType::Eof,
    //   token_value: None,
    // });
    (None, tokens)
  }

  fn peek_one(&self) -> Option<char> {
    if self.cursor + 1 >= self.code.len() {
      return None;
    }
    Some(self.code[self.cursor + 1])
  }

  fn make_number(&mut self) -> (Option<String>, String, u32) {
    let mut num: Vec<char> = Vec::new();
    let mut dot_count: u32 = 0;
    while let Some(cur) = self.get_cur() {
      if cur == '.' {
        dot_count += 1;
      }
      num.push(cur);
      if let Some(peek) = self.peek_one() {
        if !peek.is_numeric() && peek != '.' {
          break;
        }
      } else {
        break;
      }
      self.advance();
    }

    let num: String = num.into_iter().collect();

    if dot_count > 1 {
      return (Some(format!("Invalid number `{}`", num)), num, dot_count);
    }

    (None, num, dot_count)
  }

  fn get_cur(&self) -> Option<char> {
    if self.cursor >= self.code.len() {
      return None;
    }
    Some(self.code[self.cursor])
  }

  fn advance(&mut self) {
    self.cursor += 1;
  }
}