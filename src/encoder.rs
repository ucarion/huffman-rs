use std::collections::{BinaryHeap, HashMap};

#[deriving(Show)]
struct HuffTree {
    weight: int,
    node: HuffNode
}

#[deriving(Show)]
enum HuffNode {
    Tree(HuffTreeData),
    Leaf(u8)
}

#[deriving(Show)]
struct HuffTreeData {
    left: Box<HuffTree>,
    right: Box<HuffTree>
}

impl Eq for HuffTree {}
impl PartialEq for HuffTree {
    fn eq(&self, other: &HuffTree) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &HuffTree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &HuffTree) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

/// Make a Huffman encoding tree from a list of bytes (`u8`s). To use this
/// function with a string, use it as:
///
/// ```
/// let tree = make_huffman_tree(my_string.as_bytes());
/// ```
pub fn make_huffman_tree(bytes: &[u8]) -> HuffTree {
    let byte_counts = byte_frequenies(bytes);
    let trees = frequencies_to_trees(byte_counts);

    reduce_huffman_tree(trees)
}

fn byte_frequenies(bytes: &[u8]) -> HashMap<u8, int> {
    let mut byte_counts = HashMap::<u8, int>::new();
    for byte in range(0, 255u8) {
        byte_counts.insert(byte, 0);
    }

    for byte in bytes.iter() {
        match byte_counts.get_mut(byte) {
            None => {},
            Some(count) => { *count += 1 }
        }
    }

    byte_counts
}

fn frequencies_to_trees(byte_counts: HashMap<u8, int>) -> Vec<HuffTree> {
    byte_counts.iter()
        .map(|(byte, count)| HuffTree {
            weight: *count,
            node: HuffNode::Leaf(*byte)
        })
        .collect::<Vec<HuffTree>>()
}

fn reduce_huffman_tree(trees: Vec<HuffTree>) -> HuffTree {
    let mut queue = BinaryHeap::from_vec(trees);
    while queue.len() > 1 {
        let tree_a = queue.pop().unwrap();
        let tree_b = queue.pop().unwrap();

        queue.push(HuffTree {
            weight: tree_a.weight + tree_b.weight,
            node: HuffNode::Tree(HuffTreeData {
                left: box tree_a,
                right: box tree_b
            })
        });
    }

    queue.pop().unwrap()
}

fn make_encoding_table(tree: &HuffTree,
                       table: &mut HashMap<u8, String>,
                       prefix: String) {
    match tree.node {
        HuffNode::Tree(ref tree_data) => {
            make_encoding_table(&*tree_data.left, table,
                                format!("{}0", prefix));
            make_encoding_table(&*tree_data.right, table,
                                format!("{}1", prefix));
        },

        HuffNode::Leaf(byte) => {
            table.insert(byte, prefix.to_string());
        }
    }
}
