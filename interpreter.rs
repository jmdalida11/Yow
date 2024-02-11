use node::{Node, NodeType, NumberNode, BinaryOpNode};
use token::{TokenType};
use number::{Number};

pub struct Interpreter; 

impl Interpreter {
  pub fn new() -> Self {
    Self
  }

  pub fn interpret(&self, ast: Box<dyn Node>) {
    println!("{}", self.visit(&ast).to_string());
  }

  fn visit(&self, node: &Box<dyn Node>) -> Number {
    match &node.get_node_type() {
      NodeType::BinaryOp => {
        self.visit_binary_op(node.as_any().downcast_ref::<BinaryOpNode>().unwrap())
      },
      NodeType::Number => {
        self.visit_number(node.as_any().downcast_ref::<NumberNode>().unwrap())
      },
    }
  }

  fn visit_number(&self, node: &NumberNode) -> Number {
    let token = node.get_token();

    match token.token_type {
      TokenType::Integer => Number::Int(token.token_value.as_ref().unwrap().parse().unwrap()),
      TokenType::Float => Number::Float(token.token_value.as_ref().unwrap().parse().unwrap()),
      _ => panic!("Invalid Token Number Type"),
    }
  }

  fn visit_binary_op(&self, node: &BinaryOpNode) -> Number {
    let op = node.get_token().token_type;
    let left = &node.left;
    let right = &node.right;

    match op {
      TokenType::Plus => self.visit(left) + self.visit(right),
      TokenType::Minus => self.visit(left) - self.visit(right),
      TokenType::Div => self.visit(left) / self.visit(right),
      TokenType::Star => self.visit(left) * self.visit(right),
      _ => panic!("Invalid TokenType!"),
    } 
  }
}