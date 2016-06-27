use std::rc::Rc;
use std::cell::RefCell;

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

    pub fn display(&self) {
        for node in &self.nodes {
            let node = node.borrow();
            println!("{}", node.name);

            for edge in &node.edges {
                println!("|--{}--> {}", edge.weight, edge.destination.borrow().name);
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

    fn connect(&mut self, destination: NodeRef) {
        let edge = Edge::new(destination, 10);
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

pub fn generate_weighted_directed_graph() -> Graph {
    // Create nodes
    let a = Node::new("A".to_string());
    let b = Node::new("B".to_string());

    // Connect them
    {
        let mut a = a.borrow_mut();
        a.connect(b.clone());
    }

    // Add them to the graph
    let mut graph = Graph::new();
    graph.nodes.push(a);
    graph.nodes.push(b);

    graph
}
