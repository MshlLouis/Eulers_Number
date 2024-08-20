use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Div;
use std::sync::mpsc;
use std::time::Instant;
use rug::{Float, Integer};

fn main() {

    let start_time = Instant::now();
    let mut now;
    let mut elapsed_since_last_percent;
    let reps = 100000f64;             //max factorial
    let precision = ((reps + 0.5) * reps.log10()-(std::f64::consts::LOG10_E * reps) + 0.399089934) + 2.0;
    let bit_precision = 3.32193;    //don't change
    let additional_precision = (precision / reps * bit_precision).ceil() as u32;

    let values = compute_values(reps as i32, (10.0 * bit_precision) as u32, additional_precision);

    let handle2 = thread::spawn(move || {
        write_float_to_file(&values.0, "numerator.txt", true);
    });
    write_float_to_file(&values.1, "denominator.txt", true);
    handle2.join().unwrap();

    now = Instant::now();
    let numerator_str = fs::read_to_string("numerator.txt").expect("Unable to read file");
    let denominator_str = fs::read_to_string("denominator.txt").expect("Unable to read file");
    elapsed_since_last_percent = now.elapsed();
    println!("Reading files into Strings took: {:.3?}", elapsed_since_last_percent);

    let numerator = Integer::from_str_radix(&*numerator_str, 10).unwrap();
    let denominator = Integer::from_str_radix(&*denominator_str, 10).unwrap();
    let numerator_float = Float::with_val((precision*bit_precision) as u32, &numerator);
    let denominator_float = Float::with_val((precision*bit_precision) as u32, &denominator);

    now = Instant::now();
    let result = Float::with_val((precision*bit_precision) as u32, &numerator_float.div(&denominator_float));
    elapsed_since_last_percent = now.elapsed();

    println!("Calculating result took: {:.3?}", elapsed_since_last_percent);

    write_float_to_file(&result, "result.txt", false);

    println!("---------------------\nDone");
    println!("Elapsed: {:.3?}", start_time.elapsed());
}

fn write_float_to_file(number: &Float, path_name: &str, remove_dot: bool) {

    let timer = Instant::now();

    if fs::metadata(path_name).is_ok() {
        fs::remove_file(path_name).expect("TODO: panic message");
    }

    let mut data = number.to_string();

    if remove_dot {
        if let Some(pos) = data.find('.') {
            data = data[..pos].to_string();
        }
    }

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

    println!("Finished writing result in: {:.3?}", timer.elapsed());
}

fn compute_values(reps: i32, mut precision: u32, additional_precision: u32) -> (Float, Float) {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let mut final_denominator = Float::with_val(10,1);
        for i in 1..=reps {
            final_denominator *= i;
            precision += additional_precision;
            final_denominator.set_prec(precision);
        }
        tx.send(final_denominator).expect("Failed to send the result");
    });

    let mut final_numerator = Float::with_val(precision,1);
    let numerator2 = Float::with_val(precision,1);
    let start_time = Instant::now();
    let mut now = start_time;
    let mut elapsed_since_last_percent;

    for i in 1..=reps {
        if i % (reps / 100) == 0 && i > 0 {
            println!("{}%", (100f32 / (reps as f32 / i as f32)).round());
            elapsed_since_last_percent = now.elapsed();
            now = Instant::now();
            println!(
                "Elapsed: {:.3?}\nTotal elapsed: {:.3?}\n---------------------",
                elapsed_since_last_percent,
                start_time.elapsed()
            );
        }
        final_numerator = (final_numerator * i) + &numerator2;
        precision += additional_precision;
        final_numerator.set_prec(precision);
    }

    let final_denominator = rx.recv().expect("Failed to receive the result");
    handle1.join().unwrap();
    (final_numerator, final_denominator)
}