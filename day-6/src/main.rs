mod app;
use app::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

/// A node in a graph of nodes.
///
/// Each node may have zero or more children.
///
/// # Important
/// If a graph with a circular reference is create, the memory **will** leak:
/// ```rust
/// let mut node1 = GraphNode::new("A");
/// let mut node2 = GraphNode::new("B");
///
/// node1.borrow_mut().children.push(node2.clone());
/// node2.borrow_mut().children.push(node1);
/// ```
struct GraphNode {
    children: Vec<Rc<RefCell<GraphNode>>>,
    identifier: String,
}

impl GraphNode {
    fn new(identifier: String) -> Rc<RefCell<GraphNode>> {
        Rc::new(RefCell::new(GraphNode {
            children: Vec::new(),
            identifier,
        }))
    }
}

fn main() {
    // get the input file
    let opt: Opt = app().get_matches().into();

    // open the file and prepare it for parsing
    let file = File::open(opt.path).unwrap();
    let mut reader = BufReader::new(file);

    // create the graph
    let mut nodes: HashMap<String, Rc<RefCell<GraphNode>>> = HashMap::new();
    reader.lines().map(|line| line.unwrap()).map(|line| {
        let stars = line
            .split(")")
            .map(|identifier| match nodes.get(identifier) {
                Some(node) => node.clone(),
                None => {
                    let node = GraphNode::new(identifier.to_owned());
                    nodes.insert(identifier.to_owned(), node.clone());
                    node
                }
            });
        // let center_identifier = tokens.nth(0).unwrap();
        // let orbiter_identifier = tokens.nth(1).unwrap();
    });

    println!("Hello, world!");
}
