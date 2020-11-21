use bitfoo::{Addr};

fn main() {
    let max = 10_000_000;
    for bitno in 0..max {
        let addr = Addr::new(bitno);
        if bitno % 100_000 == 0 || bitno < 300 {
            println!("{} address => {:?}", bitno, addr);
        }
    }

    let bitno = 4_000_000_000 * 4_000_000_000 + 1234567;
    let addr = Addr::new(bitno);
    println!("{} address => {:?}", bitno, addr);
}

