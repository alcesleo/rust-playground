use std::rc::Rc;
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

pub struct Edge {
    weight: usize,
    destination: NodeRef,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }

    fn random_node(&self) -> NodeRef {
        thread_rng().choose(&self.nodes).unwrap().clone()
    }

    pub fn display(&self) {
        for node in &self.nodes {
            let node = node.borrow();
            println!("({})", node.name);

            for edge in &node.edges {
                println!(" |--- {:^3} ---> ({})", edge.weight, edge.destination.borrow().name);
            }

            println!("");
        }
    }
}

impl Node {
    fn new(name: String) -> NodeRef {
        // TODO: Use Weak pointers to prevent memory leaks
        Rc::new(RefCell::new(Node {
            name: name,
            edges: Vec::new(),
        }))
    }

    fn connect(&mut self, destination: NodeRef, weight: usize) {
        let edge = Edge::new(destination, weight);
        self.edges.push(edge)
    }
}

impl Edge {
    fn new(destination: NodeRef, weight: usize) -> Edge {
        Edge {
            weight: weight,
            destination: destination,
        }
    }
}

pub fn generate_weighted_directed_graph(nodes: usize, edges: usize) -> Graph {
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

        // Simply use n as the weight, this could also be randomized
        origin.borrow_mut().connect(destination.clone(), n);
    }

    graph
}
