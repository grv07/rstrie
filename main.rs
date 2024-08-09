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
            // TODO: Unable to understand why this requires
            let mut new_prefix = prefix.clone();
            // println!("{c:?}");
            new_prefix.push(c.to_owned());
            child.traverse_node(new_prefix, result);
        }
    }

    fn traverse_node_with_stack(&self, prefix: String, result: &mut Vec<String>) {
        let mut stack = vec![(prefix, self)];

        while let Some((prefix, head)) = stack.pop() {
            if head.is_terminal {
                result.push(prefix.clone());
            }

            for (c, child) in head.childs.iter() {
                // TODO: Unable to understand why this requires
                let mut new_prefix = prefix.clone();
                new_prefix.push(c.clone());
                stack.push((new_prefix, child));
            }
        }
    }

    fn _auto_complete(&self, prefix: &str) -> Vec<String> {
        let mut head = self;
        for c in prefix.chars() {
            if let Some(new_head) = head.childs.get(&c) {
                head = new_head;
            }
        }

        let mut result = vec![];
        head.traverse_node(prefix.to_string(), &mut result);

        result
    }

    fn delete_node(&mut self, mut query: impl Iterator<Item = char> + std::fmt::Debug) -> bool {
        let c = query.next();

        if self.is_terminal && c.is_none() {
            self.is_terminal = false;
            return self.childs.is_empty();
        }

        if let Some(c) = c {
            match self.childs.get_mut(&c) {
                Some(new_head) => {
                    if new_head.delete_node(query) {
                        self.childs.remove(&c);
                        return self.childs.is_empty() && !self.is_terminal;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        false
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

    fn auto_complete(&self, prefix: &str) -> Vec<String> {
        self.root._auto_complete(prefix)

        // self.root.childs.get()
    }

    fn traverse_with_stack(&self) -> Vec<String> {
        let mut result = vec![];
        let prefix = String::new();
        self.root.traverse_node_with_stack(prefix, &mut result);
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

    fn delete(&mut self, query: &str) -> bool {
        self.root.delete_node(query.chars())
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

    let q = "databaseware";
    let r = t.auto_complete(q);

    println!("{q} autocomplete is {r:?}");

    let r = t.traverse();
    println!("before droppeed recursive traverse {r:?}");

    let droped = t.delete("databasehouse");
    println!("databasehouse got droped {droped:?}");

    let r = t.traverse();
    println!("after droped recursive traverse {r:?}");

    let m = t.search("database");
    println!("database {m:?}");

    let m = t.start_with("dat".chars());
    println!("start with \"dat\" {m:?}");

    let m = t.start_with("dala".chars());
    println!("start with \"dala\" {m:?}");

    let r = t.traverse();
    println!("recursive traverse {r:?}");

    let r = t.traverse_with_stack();
    println!("traverse_with_stack {r:?}");
}
