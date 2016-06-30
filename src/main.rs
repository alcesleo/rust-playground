extern crate rand;

mod graph;

fn main() {
    let graph = graph::generate_weighted_directed_graph(5, 15);
    graph.display();
}
