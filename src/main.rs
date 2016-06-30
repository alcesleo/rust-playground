extern crate rand;

mod graph;
use graph::Graph;

fn main() {
    let nodes = 5;
    let edges = 15;
    let graph = Graph::generate(nodes, edges);

    graph.display();
}
