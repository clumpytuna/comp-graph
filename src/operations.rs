use crate::comp_graph::Node;
use crate::comp_graph::NodeRef;

// Each operation implements Operation trait. Each operation is a structure with a compute method

pub trait Operation {
    fn compute(&self, arguments: &Vec<f32>) -> f32;
}

pub fn create_input() -> NodeRef {
    Node::new(Some(0.0), Vec::new(), None, Vec::new())
}

// Binary multiplication
pub struct Mul {}

impl Operation for Mul {
    fn compute(&self, arguments: &Vec<f32>) -> f32 {
        if arguments.len() < 2 {
            panic!("Too few arguments for Multiplication operation");
        }
        arguments[0] * arguments[1]
    }
}

// Binary add
pub struct Add {}

impl Operation for Add {
    fn compute(&self, arguments: &Vec<f32>) -> f32 {
        if arguments.len() < 2 {
            panic!("Too few arguments for Add operation");
        }
        arguments[0] + arguments[1]
    }
}

// Binary pow
pub struct Pow {}

impl Operation for Pow {
    fn compute(&self, arguments: &Vec<f32>) -> f32 {
        if arguments.len() < 2 {
            panic!("Too few arguments for Pow operation");
        }
        arguments[0].powf(arguments[1])
    }
}

// Binary pow
pub struct Sin {}

impl Operation for Sin {
    fn compute(&self, arguments: &Vec<f32>) -> f32 {
        if arguments.len() < 1 {
            panic!("Too few arguments for Sin operation");
        }
        arguments[0].sin()
    }
}

// Functions to create and update graph
pub fn add(lhs: NodeRef, rhs: NodeRef) -> NodeRef {
    Node::new(None, vec![lhs, rhs], Some(Box::new(Add {})), Vec::new())
}

pub fn mul(lhs: NodeRef, rhs: NodeRef) -> NodeRef {
    Node::new(None, vec![lhs, rhs], Some(Box::new(Mul {})), Vec::new())
}

pub fn pow_f32(lhs: NodeRef, rhs: f32) -> NodeRef {
    Node::new(
        None,
        vec![lhs, Node::new(Some(rhs), Vec::new(), None, Vec::new())],
        Some(Box::new(Pow {})),
        Vec::new(),
    )
}

pub fn sin(lhs: NodeRef) -> NodeRef {
    Node::new(None, vec![lhs], Some(Box::new(Sin {})), Vec::new())
}

pub fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}
