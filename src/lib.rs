mod bitfoo;
pub use bitfoo::BitFoo;

mod node;
pub use node::Node;

mod stores;
pub use stores::{MapStore,OffsetStore,BitVecStore};

// EYE - need to test BitVecStore and MapStore to see what we
// want to use for raw bit vector stores

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
