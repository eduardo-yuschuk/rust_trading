#[derive(Debug)]
pub enum NodeType {
    Operator,
    Operand,
}

#[derive(Debug)]
pub enum Operation {
    None,
    Addition,
    Substraction,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub operation: Operation,
    pub operands: Vec<Node>,
    pub value: f32,
}

pub fn eval(node: Node) -> f32 {
    match node.node_type {
        NodeType::Operator => {
            match node.operation {
                Operation::None => panic!("Invalid Operation None on Operator Node"),
                Operation::Addition => {
                    let mut total = 0f32;
                    for operand in node.operands {
                        total = total + eval(operand);
                    }
                    return total;
                },
                Operation::Substraction => {
                    panic!("unimplemented operation");
                }
            }
        },
        NodeType::Operand => {
            return node.value;
        }
    }
}

pub fn build_sample() -> Node {
    let root = Node { 
        node_type: NodeType::Operator, 
        operation: Operation::Addition, 
        operands: vec!(
            Node {
                node_type: NodeType::Operand,
                operation: Operation::None,
                operands: vec!(),
                value: 1f32,
            },
            Node {
                node_type: NodeType::Operand,
                operation: Operation::None,
                operands: vec!(),
                value: 2f32,
            }
        ),
        value: 0f32,
    };
    return root;
}

#[cfg(test)]
mod tests {
    use { /*super::NodeType, super::Operation, super::Node, */super::build_sample, super::eval };
    #[test]
    fn it_works() {
        let root = build_sample();
        assert_eq!(eval(root), 3f32);
    }
}