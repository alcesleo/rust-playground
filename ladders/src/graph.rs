use std::collections::HashMap;

pub struct Graph<'a> {
    nodes: HashMap<String, Vec<&'a str>>
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph { nodes: HashMap::new() }
    }

    pub fn display(&self) {
        for (node, edges) in &self.nodes {
            println!("{}", node);
            for edge in edges {
                println!(" -> {}", edge);
            }
        }
    }

    pub fn construct(words: Vec<&'a String>) -> Graph<'a> {
        let mut graph = Graph::new();

        for word in &words {
            for i in 0..word.len() {
                let bucket = blank_character(word, i);

                graph.connect(bucket, word);
            }
        }

        graph
    }

    fn connect(&mut self, node: String, edge: &'a str) {
        if !self.nodes.contains_key(&node) {
            self.nodes.insert(node.clone(), Vec::new());
        }

        let edges = self.nodes.get_mut(&node).unwrap();

        edges.push(edge);
    }
}


fn blank_character(input: &str, index: usize) -> String {
    let mut result = input.to_string().into_bytes();
    result[index] = '_' as u8;
    String::from_utf8(result).unwrap()
}

#[test]
fn blanks_character() {
    assert_eq!(blank_character("test", 2), "te_t");
}
