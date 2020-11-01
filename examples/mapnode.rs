use bitfoo::nodes::MapNode;
use bitfoo::traits::BitSetLogic;

fn main() {
    let mut bm = MapNode::new();

    for i in 0..=65 {
        if i % 3 == 0 {
            // Set every 3rd bit
            bm.set(i);
        }
        if i % 2 == 0 {
            // But clear every 2nd bit
            bm.clear(i);
        }
        println!("SETTING: Is {} set ? => {}", i, bm.get(i));
    }

    for i in 0..=128 {
        println!("CHECKING: {} set ? => {}", i, bm.get(i));
    }
    bm.shrink();
}
