use crate::utils::Rectangle;
use log::{error, info, log};

#[derive(Clone)]
pub struct BSPLayout<W: Clone> {
    nodes: Vec<Node<W>>,
    max_level: usize,
}

impl<W: Clone> Default for BSPLayout<W> {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone)]
pub enum Node<W: Clone> {
    Node { split: Split, proportion: f64 },
    Leaf(W),
    Empty,
}

impl<W: Clone> Node<W> {
    pub fn is_empty(&self) -> bool {
        match self {
            Node::Empty => true,
            _ => false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Split {
    /// Up and down windows split
    Horizontal,
    /// Left and right window split
    Vertical,
}

pub fn get_side_leaf(ix: usize) -> usize {
    if (ix + 1) % 2 == 0 {
        ix + 1
    } else {
        ix - 1
    }
}

/// Get index postion to parent, return -1 on top case
pub fn get_parent(ix: usize) -> usize {
    ((ix + 1) / 2) - 1
}

impl<W: Clone> BSPLayout<W> {
    /// Creates a empty layout
    pub fn empty() -> Self {
        Self {
            nodes: vec![Node::Empty],
            max_level: 1,
        }
    }

    /// Find a node with a query a return his position into the tree
    pub fn find<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(&Node<W>) -> bool,
    {
        let mut ix = 0;
        let mut found = None;
        while ix < self.nodes.len() && found.is_none() {
            if predicate(&self.nodes[ix]) {
                found = Some(ix);
            } else {
                ix += 1;
            }
        }
        found
    }

    /// Insert a new value in the first empty node, it keeps a balanced tree
    /// O(n)
    pub fn add_leaf(&mut self, elem: W, split: Split) {
        match self.find(Node::is_empty) {
            Some(ix) => self.insert(ix, elem, split),
            None => self.insert(self.nodes.len(), elem, split),
        }
    }

    /// UNSAFE (could break internal structure logic)
    /// O(n) Almost O(1) except when is necesary to increase the vector
    pub fn insert(&mut self, ix: usize, elem: W, split: Split) {
        let parent = get_parent(ix);
        if ix == 0 {
            self.nodes[0] = Node::Leaf(elem)
        } else {
            if self.nodes.len() <= 2 * parent + 1 {
                self.increase_resolution();
            }
            self.nodes.swap(2 * parent + 1, parent);
            self.nodes[2 * parent + 2] = Node::Leaf(elem);
            self.nodes[parent] = Node::Node {
                split,
                proportion: 0.5,
            };
        }
    }

    /// UNSAFE (could break internal structure logic)
    /// O(n) Almost O(1) except when is necesary to decrease the vector
    pub fn remove(&mut self, ix: usize) {
        let parent = get_parent(ix);
        if ix == 0 {
            *self = Self::empty();
        } else {
            let sibling = get_side_leaf(ix);
            self.nodes.swap(parent, sibling);
            self.nodes[ix] = Node::Empty;
            self.nodes[sibling] = Node::Empty;
            if self.can_decrease_resolution() {
                self.decrease_resolution()
            }
        }
    }

    /// UNSAFE (vector unbounds)
    /// Try to change two leaf in the tree, at worst case doesn't do anything
    pub fn interchange_leaf(&mut self, ix: usize, ix2: usize) {
        if self.nodes[ix].is_leaf() && self.nodes[ix].is_leaf() {
            self.nodes.swap(ix, ix2)
        }
    }

    /// Gaps, decorations, borders, draggers
    /// Build squares for each leaf and node. it share the position than `self.nodes`
    pub fn build_squares(&self, root: Rectangle) -> Vec<Option<Rectangle>> {
        let mut squares = vec![Some(root)]; // same capacity as self.nodes
        let mut func = |node: &Node<W>, parent_rec_ix: usize| match node {
            Node::Node { proportion, split } => {
                let parent_rec: Rectangle = squares[parent_rec_ix].clone().expect("Parent Node");
                let mut fst = parent_rec.clone();
                let mut snd = parent_rec.clone();
                if *split == Split::Horizontal {
                    let new_height = parent_rec.height as f64 * *proportion;
                    fst.y = new_height.ceil() as u32;
                    fst.height = new_height.ceil() as u32;
                    snd.height = parent_rec.width - new_height.floor() as u32;
                } else {
                    let new_width = parent_rec.width as f64 * *proportion;
                    fst.width = new_width.ceil() as u32;
                    snd.width = new_width.ceil() as u32;
                    snd.x = parent_rec.width - new_width.floor() as u32;
                }
                squares[2 * parent_rec_ix + 1] = Some(fst);
                squares[2 * parent_rec_ix + 2] = Some(snd);
            }
            _ => return,
        };
        self.map_nodes_from(0, &mut |ix| func(&self.nodes[ix], ix));
        squares
    }

    pub fn map_nodes_from<F>(&self, ix: usize, mapper: &mut F)
    where
        F: FnMut(usize) -> (),
    {
        match self.nodes[ix] {
            Node::Node { .. } => {
                mapper(ix);
                self.map_nodes_from(2 * ix + 1, mapper);
                self.map_nodes_from(2 * ix + 2, mapper);
            }
            _ => return,
        }
    }

    // pub fn leaf_move_up(&mut self, ix: usize) {}
    // pub fn leaf_move_down(&mut self, _ix: usize) {}
    // pub fn leaf_move_left(&mut self, _ix: usize) {}
    // pub fn leaf_move_right(&mut self, _ix: usize) {}
    // /// Return a sets with windows that can be interchangeable
    // pub fn leaf_moves(&self, _ix: usize) -> Vec<usize> {
    //     vec![]
    // }

    // pub fn look_upwards_until<F>(&self, ix: usize, predicate: F) -> usize
    // where
    //     F: Fn(&Node<W>) -> bool,
    // {
    //     if predicate(&self.nodes[ix]) {
    //         ix
    //     } else if ix != 0 {
    //         self.look_upwards_until(get_parent(ix), predicate)
    //     } else {
    //         0
    //     }
    // }

    pub fn increase_resolution(&mut self) {
        self.max_level = self.max_level << 1;
        self.nodes.append(&mut vec![Node::Empty; self.max_level]);
    }

    pub fn decrease_resolution(&mut self) {
        self.nodes.truncate(self.nodes.len() - self.max_level);
        self.max_level = self.max_level >> 1
    }

    /// Improvement: Take a number of left Leaf nodes in the last level
    pub fn can_decrease_resolution(&self) -> bool {
        self.nodes[self.nodes.len() - self.max_level..]
            .iter()
            .find(|node| !node.is_empty())
            .is_some()
    }
}