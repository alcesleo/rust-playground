use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::usize;

const INF: usize = usize::MAX;

mod graph;
use graph::Graph;

fn main() {
    let max_letters = 5;
    let words = dictionary(max_letters);

    let graph = Graph::construct(&words);
    // graph.display();

    find_word_ladder(&graph, "right", "wrong");
}

fn find_word_ladder(graph: &Graph, start: &str, end: &str) {
    let mut dist: HashMap<&str, usize> = HashMap::new();
    let mut prev: HashMap<&str, &str>  = HashMap::new();
    let mut queue = LinkedList::new();

    for word in graph.nodes.keys() {
        dist.insert(word, INF);
    }

    dist.insert(start, 0);
    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for edge in graph.nodes.get(current).unwrap() {
            if *dist.get(edge).unwrap() == INF {
                let new_dist = dist.get(current).unwrap() + 1;
                dist.insert(edge, new_dist);
                prev.insert(edge, current);
                queue.push_back(edge);
            }
        }
    }

    let mut result = LinkedList::new();
    let mut current = end;
    result.push_back(end);

    while let Some(word) = prev.get(current) {
        result.push_front(word);
        current = word;
    }

    println!("{:?}", result);
}

fn dictionary(length_limit: usize) -> Vec<String> {
    let mut file   = File::open("/usr/share/dict/words").unwrap();
    let mut result = String::new();

    file.read_to_string(&mut result);

    result
        .split("\n")
        .map(str::to_string)
        .filter(|word| word.len() <= length_limit)
        .collect()
}
