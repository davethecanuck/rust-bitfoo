use std::time::{Instant,Duration};
use bitfoo::{Addr};

fn main() {
    let max = 10_000_000;
    let start = Instant::now();

    for bitno in 0..max {
        let addr = Addr::new(bitno);
        if bitno % 1_000_000 == 0 {
            println!("{} address => {:?}", bitno, addr);
        }
    }
    let rate = get_rate(start.elapsed(), max);
    println!("Set {} Addr at rate of {} addr/sec", max, rate);

    let bitno = u64::MAX;
    let addr = Addr::new(bitno);
    println!("{} address => {:?}", bitno, addr);
    println!("Size of Addr is {} bytes", std::mem::size_of::<Addr>());
}

fn get_rate(elapsed: Duration, count: u64) -> u64 {
    (count as f64 / elapsed.as_secs_f64()) as u64
}
