pub struct Graph<'a> {
    nodes: Vec<Node<'a>>,
}

pub struct Node<'a> {
    word: &'a str,
    edges: Vec<&'a str>
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph { nodes: Vec::new() }
    }

    pub fn construct(words: Vec<&'a String>) -> Graph<'a> {
        let mut graph = Graph::new();

        for word in &words {
            let mut node = Node::new(word);

            for edge in &words {
                if one_letter_off(word, edge) {
                    node.edges.push(edge);
                }
            }

            graph.nodes.push(node);
        }

        graph
    }

    pub fn display(&self) {
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
