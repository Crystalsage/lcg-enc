use std::time::{SystemTime, UNIX_EPOCH};

pub const A: u64 = 1103515245;
pub const M: u64 = 2_u64.pow(31);
pub const C: u64 = 12345;

// A state for the generator.
#[derive(Debug)]
struct LCG {
    x: Vec<u64>,
    a: u64,
    c: u64,
    m: u64,
}

impl LCG {
    // Initialize a generator with a seed
    fn init(x0: u64, a:u64, c: u64, m: u64) -> Self {
        LCG {
            x: vec![x0],
            a,
            c,
            m,
        }
    }

    // Yield the next value from the generator.
    fn next(self: &mut Self) -> u64 {
        let next_x = ((self.a * self.x.last().unwrap()) + self.c) % self.m;
        self.x.push(next_x);

        return next_x;
    }
}

fn get_time_as_unix_timestamp() -> std::time::Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("What, time travel!?");

    return since_the_epoch;
}

// Returns 'n' random numbers
pub fn get_random_numbers(n: i64) -> Vec<u64> {
    let mut random_nums: Vec<u64> = Vec::new();

    // Initialize LCG
    // x0, a, c, m
    // GCC (glibc):
    //   - m: 2^31, 
    //   - a: 1103515245,
    //   - c: 12345,
    //   - X_0: Bits 30..0
    //
    // For this implementation, the seed is the timestamp.
    let x0 = get_time_as_unix_timestamp().as_secs();

    let mut lcg: LCG = LCG::init(x0, A, C, M);

    for _ in 0..n {
        random_nums.push(lcg.next())
    }

    return random_nums;
}

