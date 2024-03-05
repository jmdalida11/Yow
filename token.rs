#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
  Integer,
  Float,
  Plus,
  Minus,
  Div,
  Star,
  OpenParen,
  CloseParen,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub token_value: Option<String>,
}