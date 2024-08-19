use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use num_bigint::BigInt;

fn main() {                     //THIS PROGRAM ONLY GENERATES THE NUMERATOR AND DENOMINATOR,
                                //THE FINAL CALCULATION HAS TO BE DONE IN A SEPARATE PROGRAM
    let start_time = Instant::now();
    let mut now = Instant::now();
    let mut elapsed_since_last_percent;
    let reps = 100_000;         //max factorial, number of precise decimals will be (#reps!)
    let numerator2 = BigInt::from(1);
    let mut final_numerator: BigInt = BigInt::from(1);
    let mut final_denominator: BigInt = BigInt::from(1);

    for i in 1..=reps {
        if i % (reps/100) == 0 && i>0
        {
            println!("{}%", 100f32 /(reps as f32/i as f32));
            elapsed_since_last_percent = now.elapsed();
            now = Instant::now();
            println!("Elapsed: {:.3?}\nTotal elapsed: {:.3?}", elapsed_since_last_percent, start_time.elapsed());
        }
        final_numerator = (final_numerator * i) + &numerator2;
        final_denominator *= i;
    }
    write_file(&mut final_numerator, "numerator.txt");
    write_file(&mut final_denominator, "denominator.txt");

    println!("--------------\nDone");
    println!("Elapsed: {:.3?}", start_time.elapsed());
}

fn write_file(number: &BigInt, path_name: &str) {

    if fs::metadata(path_name).is_ok() {
        fs::remove_file(path_name).expect("TODO: panic message");
    }

    let data = number.to_string();
    let data_split_count = data.len() / 100;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_name)
        .unwrap();

    for i in 0..data_split_count {
        let out = (&data[i * 100..(i + 1) * 100]).to_owned() + "\n";
        file.write(out.as_bytes()).expect("Error");
    }

    let out = (&data[data_split_count * 100..data.len()]).to_owned() + "\n";
    file.write(out.as_bytes()).expect("Error");

    println!("Finished writing file");
}