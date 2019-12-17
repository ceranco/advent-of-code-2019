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
#[derive(Debug)]
struct GraphNode {
    children: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    fn new() -> Rc<RefCell<GraphNode>> {
        Rc::new(RefCell::new(GraphNode {
            children: Vec::new(),
        }))
    }

    fn traverse<F: FnMut(i32)>(node: Rc<RefCell<GraphNode>>, proc: F) {
        fn traverse_impl<F: FnMut(i32)>(node: Rc<RefCell<GraphNode>>, depth: i32, proc: Rc<RefCell<F>>) {
            // run the proc with the current depth
            (&mut *proc.borrow_mut())(depth);

            // traverse all the children
            for child in node.borrow().children.iter() {
                traverse_impl(child.clone(), depth + 1, proc.clone());
            }
        };
        traverse_impl(node, 0, Rc::new(RefCell::new(proc)));
    }
}

fn main() {
    // get the input file
    let opt: Opt = app().get_matches().into();

    // open the file and prepare it for parsing
    let file = File::open(opt.path).unwrap();
    let reader = BufReader::new(file);

    // create the graph
    let mut nodes: HashMap<String, Rc<RefCell<GraphNode>>> = HashMap::new();
    for line in reader.lines() {
        let mapping = line.unwrap();
        let mut stars = mapping
            .split(")")
            .map(|identifier| match nodes.get(identifier) {
                Some(node) => node.clone(),
                None => {
                    let node = GraphNode::new();
                    nodes.insert(identifier.to_owned(), node.clone());
                    node
                }
            });

        let center = stars.next().unwrap();
        let orbiter = stars.next().unwrap();
        center.borrow_mut().children.push(orbiter);
    }

    // get the center
    let com = nodes.remove("COM").unwrap();

    // recursive traveral
    let mut counter: i32 = 0;
    GraphNode::traverse(com, |depth| counter += depth);

    println!("Checksum: {}", counter);
}
