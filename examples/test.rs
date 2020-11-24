#[allow(dead_code)]
// Brute force byte array of fixed size
pub enum ByteArray {
    B0,
    B1(Box<[u8;1]>),
    B2(Box<[u8;2]>),
    B3(Box<[u8;3]>),
    B4(Box<[u8;5]>),
    B5(Box<[u8;8]>),
    // EYE - repeat up to 255?
}

#[allow(dead_code)]
enum NodeType {
    Branch(Option<Vec<BranchNode>>),
    Leaf(Option<Vec<LeafNode>>),
    Ones
}

// EYE - What if we allocated 4KB arrays of pointers or
// bits as boxed chunks, and stored the offsets into them
// so as to get slices... Complexity on resize...

// Branches are a bit bigger, but 
// allow smaller leafs
#[allow(dead_code)]
#[repr(packed)]
pub struct BranchNode {
    key: u8,
    branch_children: Option<Vec<BranchNode>>, // 24 bytes
    leaf_children: Option<Vec<LeafNode>>,     // 24 bytes
    ones_right: bool,                         // 8 bytes (word alignment)
}

#[allow(dead_code)]
#[repr(packed)]
pub struct BranchNode2 {
    key: u8,
    children: NodeType,   // 32 bytes
    ones_right: bool,     // 8 bytes (word alignment)
}

#[allow(dead_code)]
#[repr(packed)]
pub struct BranchNode3 {
    key: u8,           // 1 byte
    children: NodeType,   // 32 bytes
}

// Allows all of the children to take up way
// less space
#[allow(dead_code)]
// #[repr(packed)]            // NOT RECOMMENDED...
pub struct LeafNode {
    key: u8,            // 1 byte, but padded to 8 if not packed
    bits: u64,             // 8 byte
    // What if we used vec of keys by offset outside of the node?
    // Possibly better for padding, but worse for locality
}

// How about packing data into arrays in the NodeVec instead?
// Space is wasted at the vector level and not the individual
// node level. Nodes can be synthesized as needed as tuples 
// of offsets or similar
pub enum NodeVecType {        
    Branch(Vec<NodeVecType>),
    Leaf(Vec<LeafNodeVec>),
    Ones
}

#[allow(dead_code)]
pub struct LeafNodeVec {
    keys: Vec<u8>,      // Vector of keys - offset matches bits vec
    bits: Vec<u64>,     // Each Vec is 24 bytes minimum
}

#[allow(dead_code)]
pub struct BranchNodeVec {
    keys: Vec<u8>,      
    children: NodeVecType, // Needs to be enum of NodeVec::Branch/Leaf
}

fn main() {
    println!("Size of NodeType is {}-bytes", 
             std::mem::size_of::<NodeType>());
    println!("Size of LeafNode is {}-bytes", 
             std::mem::size_of::<LeafNode>());
    println!("Size of BranchNode is {}-bytes", 
             std::mem::size_of::<BranchNode>());
    println!("Size of BranchNode2 is {}-bytes", 
             std::mem::size_of::<BranchNode2>());
    println!("Size of BranchNode3 is {}-bytes", 
             std::mem::size_of::<BranchNode3>());

    println!("Size of ByteArray is {}-bytes", 
             std::mem::size_of::<ByteArray>());
    println!("Size of ByteArray::B1 is {}-bytes", 
             std::mem::size_of_val(&ByteArray::B1));
    println!("Size of ByteArray::B2 is {}-bytes", 
             std::mem::size_of_val(&ByteArray::B2));
    println!("Size of ByteArray::B3 is {}-bytes", 
             std::mem::size_of_val(&ByteArray::B3));

    let c = ByteArray::B3(Box::new([0;3]));
    println!("Size of c is {}-bytes", 
             std::mem::size_of_val(&c));

    println!("Size of NodeVecType is {}-bytes", 
             std::mem::size_of::<NodeVecType>());
    println!("Size of LeafNodeVec is {}-bytes", 
             std::mem::size_of::<LeafNodeVec>());
    println!("Size of BranchNodeVec is {}-bytes", 
             std::mem::size_of::<BranchNodeVec>());
}
