mod token;
mod node;
mod number;
mod lexer;
mod parser;
mod interpreter;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn get_input() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input
}

fn main() {
  loop {
    print!("> ");
    io::stdout().flush().unwrap();
    let input = get_input();

    if input.trim() == "exit" {
      break;
    }
    
    let mut lexer = Lexer::new(input);
    let (error, tokens) = lexer.tokenized();

    if let Some(err) = error {
      println!("Syntax Error: {}", err);
    } else {
      let mut parser = Parser::new(tokens);
      let ast = parser.parse();
      let interpreter = Interpreter::new();
      interpreter.interpret(ast);
    }
  }
}
