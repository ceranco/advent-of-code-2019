use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct TreeBuilder {
    root: String,
    node_children: HashMap<String, Vec<String>>,
}

impl TreeBuilder {
    pub fn new(root: String) -> Self {
        let mut node_children = HashMap::new();
        node_children.insert(root.clone(), Vec::new());
        Self {
            root,
            node_children,
        }
    }

    pub fn add_node(&mut self, parent: String, child: String) -> &mut Self {
        let vec = match self.node_children.get_mut(&parent) {
            Some(children) => children,
            None => {
                self.node_children.insert(parent.clone(), Vec::new());
                self.node_children.get_mut(&parent).unwrap()
            }
        };
        vec.push(child);
        self
    }

    pub fn build(self) -> Tree {
        let mut nodes = Vec::with_capacity(self.node_children.keys().len());
        let mut node_indices = HashMap::new();

        // this function "flattens" the tree into a vector containing all the nodes (and their depth)
        // and a `HashMap` with maps their names to their indices
        fn add_to_vector(
            nodes: &mut Vec<(String, Option<String>, i32)>,
            indices: &mut HashMap<String, usize>,
            node_children: &HashMap<String, Vec<String>>,
            root: String,
            parent: Option<String>,
            depth: i32,
        ) {
            // insert the current root into the vector and map
            indices.insert(root.clone(), nodes.len());
            nodes.push((root.clone(), parent, depth));

            if let Some(children) = node_children.get(&root) {
                for node in children {
                    add_to_vector(
                        nodes,
                        indices,
                        node_children,
                        node.clone(),
                        Some(root.clone()),
                        depth + 1,
                    );
                }
            }
        }
        add_to_vector(
            &mut nodes,
            &mut node_indices,
            &self.node_children,
            self.root.clone(),
            None,
            0,
        );

        // now we want to transform the vector containing the names into
        // one that contains the actual nodes
        let nodes = nodes
            .iter()
            .map(|(name, parent, depth)| TreeNode {
                depth: *depth,
                parent: match parent {
                    Some(name) => Some(*node_indices.get(name).unwrap()),
                    None => None,
                },
                children_indices: match self.node_children.get(name) {
                    Some(children) => children
                        .iter()
                        .map(|child_name| *node_indices.get(child_name).unwrap())
                        .collect::<Vec<_>>(),
                    None => Vec::new(),
                },
            })
            .collect();

        Tree {
            root: self.root,
            indices: node_indices,
            nodes,
        }
    }
}

pub struct Tree {
    root: String,
    indices: HashMap<String, usize>,
    nodes: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct TreeNode {
    parent: Option<usize>,
    pub depth: i32,
    children_indices: Vec<usize>,
}

impl Tree {
    pub fn distance(&self, src: &str, dst: &str) -> Option<i32> {
        let idx1 = self.indices.get(src);
        let idx2 = self.indices.get(dst);
        if idx1 == None || idx2 == None {
            return None;
        }

        let mut distance = 0;
        let mut idx1 = *idx1.unwrap();
        let mut idx2 = *idx2.unwrap();

        while idx1 != idx2 {
            let node1 = self.nodes.get(idx1).unwrap();
            let node2 = self.nodes.get(idx2).unwrap();

            if node1.depth > node2.depth {
                idx1 = node1.parent.unwrap();
                distance += 1;
            } else if node1.depth < node2.depth {
                idx2 = node2.parent.unwrap();
                distance += 1;
            } else {
                idx1 = node1.parent.unwrap();
                idx2 = node2.parent.unwrap();
                distance += 2;
            }

        }

        Some(distance)
    }

    pub fn traverse<F: FnMut(&TreeNode)>(&self, f: F) {
        self.traverse_impl(self.indices[&self.root], Rc::new(RefCell::new(f)));
    }

    fn traverse_impl<F: FnMut(&TreeNode)>(&self, root: usize, f: Rc<RefCell<F>>) {
        let node = self.nodes.get(root).unwrap();
        (&mut *f.borrow_mut())(node);

        for child in node.children_indices.iter() {
            self.traverse_impl(*child, f.clone());
        }
    }
}
