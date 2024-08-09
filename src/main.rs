use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
struct Node {
    childs: HashMap<char, Node>,
    is_terminal: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            childs: HashMap::default(),
            is_terminal: false,
        }
    }

    fn traverse_node(&self, prefix: String, result: &mut Vec<String>) {
        if self.is_terminal {
            // println!("{prefix:?}");
            result.push(prefix.clone());
        }

        for (c, child) in self.childs.iter() {
            let mut new_prefix = prefix.clone();
            new_prefix.push(c.to_owned());
            child.traverse_node(new_prefix, result);
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Trie {
    root: Node,
}

impl Trie {
    fn new(root: Node) -> Self {
        Self { root }
    }

    fn insert(&mut self, word: &str) {
        let mut head = &mut self.root;

        for c in word.chars() {
            head = head.childs.entry(c).or_insert_with(Node::new);
        }

        head.is_terminal = true;
    }

    fn traverse(&self) -> Vec<String> {
        let mut result = vec![];
        let prefix = String::new();
        self.root.traverse_node(prefix, &mut result);
        result
    }

    fn search(&self, query: &str) -> bool {
        let mut head = &self.root;

        for c in query.chars() {
            match head.childs.get(&c) {
                Some(new_head) => head = new_head,
                None => return false,
            }
        }

        return head.is_terminal;
    }

    fn start_with(&self, query: impl Iterator<Item = char>) -> bool {
        let mut head = &self.root;
        for c in query {
            match head.childs.get(&c) {
                Some(new_head) => head = new_head,
                None => return false,
            }
        }

        return true;
    }
}

fn main() {
    let mut t = Trie::new(Node::default());

    t.insert("data");
    t.insert("database");
    t.insert("databaseware");
    t.insert("databasewarehouse");
    t.insert("databasehouse");
    t.insert("datahouse");
    t.insert("string");

    // println!("{t:#?}");

    let m = t.search("database");
    println!("database {m:?}");

    let m = t.start_with("dat".chars());
    println!("start with \"dat\" {m:?}");

    let m = t.start_with("dala".chars());
    println!("start with \"dala\" {m:?}");

    // let mut s = String::new();
    // let b = String::new();
    let r = t.traverse();
    println!("{r:?}");
}
