use std::rc::{Rc, Weak};
use std::cell::RefCell;

use rand::{thread_rng, Rng};

pub struct Graph {
    nodes: Vec<NodeRef>,
}

pub struct Node {
    name: String,
    edges: Vec<Edge>,
}

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

pub struct Edge {
    weight: usize,
    destination: WeakNodeRef,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }

    fn random_node(&self) -> &NodeRef {
        thread_rng().choose(&self.nodes).unwrap()
    }

    pub fn display(&self) {
        for node in &self.nodes {
            let node = node.borrow();
            println!("({})", node.name);

            for edge in &node.edges {
                let destination = edge.destination.upgrade().unwrap();
                println!(" |--- {:^3} ---> ({})",
                         edge.weight,
                         destination.borrow().name);
            }

            println!("");
        }
    }

    pub fn generate(nodes: usize, edges: usize) -> Graph {
        let mut graph = Graph::new();

        // Create nodes
        for n in 1..nodes + 1 {
            let node = Node::new(n.to_string());
            graph.nodes.push(node);
        }

        let graph = graph; // Don't need to mutate graph anymore

        // Create edges until desired sparseness has been reached
        for n in 1..edges + 1 {
            let origin = graph.random_node();
            let destination = graph.random_node();

            // TODO: Make sure to not connect a node to itself,
            // or a node it is already connected to

            // Simply use n as the weight, this could also be randomized
            origin.borrow_mut().connect(destination, n);
        }

        graph
    }
}

impl Node {
    fn new(name: String) -> NodeRef {
        Rc::new(RefCell::new(Node {
            name: name,
            edges: Vec::new(),
        }))
    }

    fn connect(&mut self, destination: &NodeRef, weight: usize) {
        let edge = Edge::new(Rc::downgrade(destination), weight);
        self.edges.push(edge)
    }
}

impl Edge {
    fn new(destination: WeakNodeRef, weight: usize) -> Edge {
        Edge {
            weight: weight,
            destination: destination,
        }
    }
}
