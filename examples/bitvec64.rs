use std::time::{Instant,Duration};
use bitfoo::{BitVec64};

fn main() {
    let max:u64 = 100_000;
    let count:u64 = max * 64;
    let mut vec = BitVec64(0);

    // SET
    let start = Instant::now();
    for _outer in 0..max {
        for inner in 0..=63 {
            vec.set(inner);
        }
    }
    let rate = get_rate(start.elapsed(), max * 64);
    println!("SET {} bits @ {} bits/sec", count, rate);

    // GET
    let start = Instant::now();
    for _outer in 0..max {
        for inner in 0..=63 {
            let _a = vec.get(inner);
        }
    }
    let rate = get_rate(start.elapsed(), max * 64);
    println!("GET {} bits @ {} bits/sec", count, rate);

    // GET with []]
    let start = Instant::now();
    for _outer in 0..max {
        for inner in 0..=63 {
            let _a = vec[inner];
        }
    }
    let rate = get_rate(start.elapsed(), max * 64);
    println!("GET[] {} bits @ {} bits/sec", count, rate);

    // ITERATE
    for step in &[1,2,5,10,25,100,200] {
        let mut vec = BitVec64(0);
        for i in 0..=63 {
            if  i % step == 0 {
                vec.set(i);
            }
        }

        let start = Instant::now();
        for _outer in 0..max {
            for _off in vec.iter() {
                if  _off == 99 {
                    // Just putting a placeholder operation
                    // in to avoid compiler optimizing away
                    // this iterator
                    continue;
                }
            }
        }

        let rate = get_rate(start.elapsed(), max * 64);
        println!("ITERATE {} bits (step={:#3}) @ {:#.0} bits/sec", 
                 count, step, rate);
    }
}

fn get_rate(elapsed: Duration, count: u64) -> u64 {
    (count as f64 / elapsed.as_secs_f64()) as u64
}

