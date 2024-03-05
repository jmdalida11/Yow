use token::{Token, TokenType};
use node::{Node, NumberNode, BinaryOpNode};

pub struct Parser {
  tokens: Vec<Token>, 
  cursor: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      cursor: 0,
    }
  }

  fn advance(&mut self) {
    self.cursor += 1;
  }

  fn get_token(&self) -> Option<Token> {
    if self.cursor >= self.tokens.len() {
      return None;
    }

    Some(self.tokens[self.cursor].clone())
  }

  pub fn parse(&mut self) -> Box<dyn Node> {
    return self.get_expr();
  } 

  fn get_atom(&mut self) -> Box<dyn Node> {
    if let Some(token) = self.get_token() {
      match token.token_type {
        TokenType::Integer | TokenType::Float => {
          self.advance();
          return Box::new(NumberNode {
            token,
          });
        },
        TokenType::OpenParen => {
          self.advance();
          let expr = self.get_expr();
          if self.get_token().unwrap().token_type != TokenType::CloseParen {
            panic!("Close parenthesis is missing");
          }
          self.advance();
          return expr;
        },
        _ => {
          panic!("Invalid number node");
        }
      }
    }

    panic!("Invalid number node");
  }

  fn get_term(&mut self) -> Box<dyn Node> {
    let mut left = self.get_atom();

    while let Some(token) = self.get_token() {
      if token.token_type != TokenType::Div && token.token_type != TokenType::Star {
        break;
      }

      let op = token; 
      self.advance();
      let right = self.get_atom();
      left = Box::new(BinaryOpNode {
        token: op,
        left,
        right,
      });
    }

    return left;
  }

  fn get_expr(&mut self) -> Box<dyn Node> {
    let mut left = self.get_term(); 

    while let Some(token) = self.get_token() {
      if token.token_type != TokenType::Plus && token.token_type != TokenType::Minus {
        break;
      }
      let op = token;
      self.advance();
      let right = self.get_term();
      left = Box::new(BinaryOpNode {
        token: op,
        left,
        right,
      });
    }

    return left;
  }
}