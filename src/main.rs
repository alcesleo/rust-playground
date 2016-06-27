mod graph;

fn main() {
    let graph = graph::generate_weighted_directed_graph();
    graph.display();
}
