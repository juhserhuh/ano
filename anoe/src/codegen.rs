use crate::parser::ASTNode;

pub fn generate(ast: &ASTNode) {
    match ast {
        ASTNode::Program(nodes) => {
            for node in nodes {
                generate(node);
            }
        }
        ASTNode::Input(data) => {
            println!("Input: {}", data);
        }
        ASTNode::Output(data) => {
            println!("Output: {}", data);
        }
    }
}
