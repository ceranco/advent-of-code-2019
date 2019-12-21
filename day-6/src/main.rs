mod app;
mod tree;
use app::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use tree::*;

fn main() {
    // get the input file
    let opt: Opt = app().get_matches().into();

    // open the file and prepare it for parsing
    let file = File::open(opt.path).unwrap();
    let reader = BufReader::new(file);

    // create the graph
    let mut tree_builder = TreeBuilder::new(String::from("COM"));
    for line in reader.lines() {
        let mapping = line.unwrap();
        let mut stars = mapping.split(")").map(|string| string.to_owned());
        tree_builder.add_node(stars.next().unwrap(), stars.next().unwrap());
    }
    let tree = tree_builder.build();

    // recursive traveral
    let mut counter: i32 = 0;
    tree.traverse(|node| counter += node.depth);

    println!("Checksum: {}", counter);
}
