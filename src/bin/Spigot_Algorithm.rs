use std::time::Instant;

fn main() {
    const REPS: usize = 110000;                 //factorial
    const DECS: usize = 500000;                 //precise decimals
    let now = Instant::now();

    let now = Instant::now();
    let mut out = "2.".to_string();

    let mut b = [0; REPS-2];
    let mut prev = [1; REPS];
    let mut carry = 0;
    let mut final_carry: i8 = 0;
    let mut insert;

    for i in 2..REPS {
        b[i-2] = i;
    }

    for _ in 0..DECS {
        for i in (0..b.len()).rev() {
            insert = prev[i] * 10 + carry;
            prev[i] = insert % b[i];
            carry = insert / b[i];
            final_carry = carry as i8;
        }
        out += &final_carry.to_string();
    }

    println!("{}", out);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
}