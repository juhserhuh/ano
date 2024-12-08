use crate::lexer::{Token, Token::*};

#[derive(Debug)]
pub enum ASTNode {
    Input(String),
    Output(String),
    Program(Vec<ASTNode>),
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut ast = ASTNode::Program(Vec::new());
    let mut current = 0;

    while current < tokens.len() {
        match &tokens[current] {
            Input(data) => {
                ast = ASTNode::Program(vec![ASTNode::Input(data.clone())]);
                current += 1;
            }
            Output(data) => {
                ast = ASTNode::Program(vec![ASTNode::Output(data.clone())]);
                current += 1;
            }
            _ => current += 1,
        }
    }

    ast
}
