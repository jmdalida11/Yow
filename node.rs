use std::any::Any;
use token::{Token};

#[derive(Debug)]
pub enum NodeType {
  Number,
  BinaryOp,
}

pub trait Node {
  fn get_token(&self) -> &Token;
  fn print_node(&self) -> String;
  fn get_node_type(&self) -> NodeType;
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct NumberNode {
  pub token: Token,
}

#[derive(Debug)]
pub struct BinaryOpNode {
  pub token: Token,
  pub left: Box<dyn Node>,
  pub right: Box<dyn Node>,
}

impl std::fmt::Debug for dyn Node{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.print_node())
  }
}

impl Node for NumberNode {
  fn get_token(&self) -> &Token {
    return &self.token;
  }

  fn get_node_type(&self) -> NodeType {
    return NodeType::Number;
  }

  fn print_node(&self) -> String {
    format!("{:#?}", self)
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Node for BinaryOpNode{
  fn get_token(&self) -> &Token {
    return &self.token;
  }

  fn get_node_type(&self) -> NodeType {
    return NodeType::BinaryOp;
  }

  fn print_node(&self) -> String {
    format!("{:#?}", self)
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}