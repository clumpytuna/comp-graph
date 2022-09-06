use crate::operations::Operation;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::rc::Weak;

// A graph is implemented as nodes connected by references. Each node holds a reference to
// a polymorph structure, that helps to execute graph commands. A graph holds a vec of edges
// to be able to compute operations with arbitrary number of arguments.

pub struct Node {
    data: Option<f32>,
    // Nodes that needs to compute a node value
    nodes_to_compute: Vec<NodeRef>,
    operation: Option<Box<dyn Operation>>,
    // Nodes that rely on our node to compute their value.
    // Only nodes necessary to start a traverse
    nodes_depend_on_us: Vec<Weak<RefCell<Node>>>,
}

impl Node {
    fn new_node(
        data: Option<f32>,
        nodes_to_compute: Vec<NodeRef>,
        operation: Option<Box<dyn Operation>>,
        nodes_depend_on_us: Vec<Weak<RefCell<Node>>>,
    ) -> NodeRef {
        NodeRef::new(Node {
            data,
            nodes_to_compute,
            operation,
            nodes_depend_on_us,
        })
    }

    pub fn new(
        data: Option<f32>,
        nodes_to_compute: Vec<NodeRef>,
        operation: Option<Box<dyn Operation>>,
        nodes_depend_on_us: Vec<Weak<RefCell<Node>>>,
    ) -> NodeRef {
        let new = Node::new_node(data, nodes_to_compute, operation, nodes_depend_on_us);

        for n in &new.borrow().nodes_to_compute {
            n.clone().borrow_mut().push_depended_node(&new.ref_);
        }

        new
    }
}

#[derive(Clone)]
pub struct NodeRef {
    ref_: Rc<RefCell<Node>>,
}

impl NodeRef {
    pub fn new(node: Node) -> NodeRef {
        NodeRef {
            ref_: Rc::new(RefCell::new(node)),
        }
    }

    pub fn borrow(&self) -> Ref<Node> {
        self.ref_.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<Node> {
        self.ref_.borrow_mut()
    }

    pub fn compute(&mut self) -> f32 {
        self.borrow_mut().compute()
    }

    pub fn set(&mut self, value: f32) {
        self.ref_.borrow_mut().data = Some(value);
        self.ref_.borrow_mut().propagate_invalidation();
    }
}

impl Node {
    pub fn push_depended_node(&mut self, node_ref: &Rc<RefCell<Node>>) {
        self.nodes_depend_on_us.push(Rc::downgrade(node_ref));
    }

    pub fn invalidate(&mut self) {
        // If no operation in a node, it's an input or a constant node
        match &self.operation {
            Some(_) => self.data = None,
            None => (),
        }
    }

    pub fn propagate_invalidation(&mut self) {
        for n in &self.nodes_depend_on_us {
            match n.upgrade() {
                Some(ref_node) => {
                    ref_node.borrow_mut().invalidate();
                    ref_node.borrow_mut().propagate_invalidation()
                }
                None => continue,
            }
        }
    }

    // Get values from edges we depend and pass it as arguments in node's function
    fn collect_values(&self) -> Vec<f32> {
        let mut answer = Vec::new();
        for node_ref in &self.nodes_to_compute {
            let x: Option<f32> = node_ref.borrow().data;
            match x {
                Some(val) => answer.push(val),
                None => answer.push(node_ref.clone().borrow_mut().compute()),
            };
        }
        answer
    }

    pub fn compute(&mut self) -> f32 {
        match self.data {
            Some(f) => return f,
            None => match &self.operation {
                Some(op) => {
                    let result = op.compute(&self.collect_values());
                    self.data = Some(result);
                    result
                }
                // if there is no an operation in a node, that means we are in InputNode
                // if we have reach this branch that means there is no value in InputNode
                None => panic!("Uninitialized input value"),
            },
        }
    }
}
