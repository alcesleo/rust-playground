use std::fs::File;
use std::io::{BufReader, BufRead};

mod graph;
use graph::Graph;

fn main() {
    let dict = dictionary();
    let num_letters = 5;

    let words: Vec<_> = dict.iter()
                            .filter(|word| word.len() == num_letters)
                            .skip(7000)
                            .take(20)
                            .collect();

    println!("{}", words.len());

    let graph = Graph::construct(words);
    graph.display();
}

fn dictionary() -> Vec<String> {
    let f          = File::open("/usr/share/dict/words").unwrap();
    let file       = BufReader::new(f);
    let mut result = Vec::new();

    for line in file.lines() {
        result.push(line.unwrap());
    }

    result
}
