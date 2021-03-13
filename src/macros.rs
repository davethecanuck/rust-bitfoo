#[macro_export]
macro_rules! bitfoo {
    ($($x: expr),*) => {{
        let mut bv = BitFoo::new();
        $(bv.set($x);)*
        bv
    }}
}
