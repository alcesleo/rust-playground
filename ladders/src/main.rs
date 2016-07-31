use std::fs::File;
use std::io::Read;

mod graph;
use graph::Graph;

fn main() {
    let max_letters = 5;
    let words = dictionary(max_letters);

    let graph = Graph::construct(&words);
    graph.display();
}

fn dictionary(length_limit: usize) -> Vec<String> {
    let mut file   = File::open("/usr/share/dict/words").unwrap();
    let mut result = String::new();

    file.read_to_string(&mut result);

    result
        .split("\n")
        .map(str::to_string)
        .filter(|word| word.len() == length_limit)
        .collect()
}
