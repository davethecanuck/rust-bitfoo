use bitfoo::{Addr};

fn main() {
    for bitno in 0..600 {
        let addr = Addr::new(bitno);
        println!("{} address => {:?}", bitno, addr);
        //EYE - not quite working...
        // - let Addr be calculated for any bitno
        // and it will determine the required level
    }
}

