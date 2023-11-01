use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Utc};
pub fn do_it(){
   closure_no_params();
    closure_one_params();
    closure_many_params();
    closure_multiple_statements();
    closure_infer_type();
}

fn closure_no_params(){
    let get_timestamp = || -> DateTime<Utc> {Utc::now()};
    println!("Timestamp: {}", get_timestamp());
}

fn closure_one_params() {
    let reciprocal = |n: f64| -> f64 { if n == 0.0 {0.0} else { 1.0 / n } };
    println!("Reciprocal: {}", reciprocal(5.0));
}

fn closure_many_params(){
    let prod = |a: i32, b: i32| -> i32 { a * b };
    println!("Product: {}", prod(20,5));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds: u64| -> DateTime<Utc>{
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };
    println!("Timestamp: {}", get_timestamp_after_delay(5).format("%T"));
}

fn closure_infer_type() {
    let get_time_stamp = || Utc::now();
    let reciprocal = |n| if n == 0.0 {0.0} else { 1.0/n };
    println!("Timestamp: {} ", get_time_stamp());
    println!("Reciprocal: {}", reciprocal(5.0));
}
