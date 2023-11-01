pub fn do_it(){
    capture_immutable_reference();
    capture_mutable_reference();
}

fn capture_immutable_reference() {
    let os1 = "Android";
    let os2 = "IOS";

    let get_os = |product: &str| {
        if &product[..1] == "I" {os2} else { os1 }
    };

    println!("Operating System: {}", get_os("IPhone 15"));
    println!("{} \n {}", os1, os2) // we still retain ownership
}

fn capture_mutable_reference() {
    let mut os1 = String::from("Android");
    let mut os2 = String::from("IOS");

    let mut print_product_info = |product: String| {
        os1.push_str(" Operating System");
        os2.push_str(" Operating System");
        if &product[..1] == "I" { println!("{} => {}", product, os2)} else { println!("{} => {}", product, os1) }
    };

    print_product_info(String::from("IPhone"));
    println!("{} \n {}", os1, os2) // we still retain ownership
}
