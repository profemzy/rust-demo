pub fn do_it() {
    println!("\nIn passing_values::do_it()");

    let n = 42;
    let s  = String::from("hello");

    some_func(n, s);

    println!("n: {}", n);
}

fn some_func(my_int: i32, my_string: String) {
    println!("In some_func, my_int is {}", my_int);
    println!("In some_func, my_string is {}", my_string);
}