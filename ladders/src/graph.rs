use std::collections::HashMap;

pub struct Graph<'a> {
    pub nodes: HashMap<String, Vec<&'a str>>
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

    pub fn construct(words: &'a Vec<String>) -> Graph<'a> {
        // First create a graph where a bucket id (a word with a blank
        // character, e.g. "c_t") has edges to each word that fits
        // into that bucket
        let mut bucket_graph = Graph::new();

        for word in words {
            for i in 0..word.len() {
                let bucket = blank_character(word, i);

                bucket_graph.connect(bucket, word);
            }
        }

        // Create the actual graph by going through the buckets and
        // connecting every word to its neighbours
        let mut graph = Graph::new();

        for edges in bucket_graph.nodes.values() {
            for edge1 in edges {
                for edge2 in edges {
                    if edge1 != edge2 {
                        graph.connect(edge1.to_string(), edge2);
                    }
                }
            }
        }

        graph
    }

    fn connect(&mut self, node: String, edge: &'a str) {
        self.nodes
            .entry(node)
            .or_insert_with(Vec::new)
            .push(edge);
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
