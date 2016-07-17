use std::fs::File;
use std::io::{BufReader, BufRead};

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

fn one_letter_off(a: &str, b: &str) -> bool {
    if a.len() != b.len() { return false }

    let diff_count =
        a.chars()
        .zip(b.chars())
        .filter(|pair| pair.0 != pair.1)
        .count();

    diff_count == 1
}

#[test]
fn exactly_one_letter_off() {
    assert!(one_letter_off("man", "map"));
}

#[test]
fn different_length_word() {
    assert!(!one_letter_off("maps", "map"));
}

#[test]
fn two_letters_off() {
    assert!(!one_letter_off("men", "map"));
}

struct Graph<'a> {
    nodes: Vec<Node<'a>>,
}

struct Node<'a> {
    word: &'a str,
    edges: Vec<&'a str>
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph { nodes: Vec::new() }
    }

    fn construct(words: Vec<&'a String>) -> Graph<'a> {
        let mut graph = Graph::new();

        for word in &words {
            let mut node = Node::new(word);

            for edge in &words {
                if one_letter_off(word, edge) {
                    node.edges.push(word);
                }
            }

            graph.nodes.push(node);
        }

        graph
    }

    fn display(&self) {
        for node in &self.nodes {
            println!("{}", node.word);
            for edge in &node.edges {
                println!(" -> {}", edge);
            }
        }
    }
}

impl<'a> Node<'a> {
    fn new(word: &str) -> Node {
        Node { word: word, edges: Vec::new() }
    }

}
