use std::fs::File;
use std::io::Read;

mod graph;
use graph::Graph;

fn main() {
    let dict = dictionary();
    let num_letters = 5;

    let words: Vec<_> = dict.iter()
                            .filter(|word| word.len() == num_letters)
                            .collect();

    println!("{}", words.len());

    let graph = Graph::construct(words);
    graph.display();
}

fn dictionary() -> Vec<String> {
    let mut file   = File::open("/usr/share/dict/words").unwrap();
    let mut result = String::new();

    file.read_to_string(&mut result);

    result.split("\n").map(str::to_string).collect()
}
