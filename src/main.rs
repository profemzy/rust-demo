fn main() {
    println!("Hello, world!");
    demo_integers();
}

fn demo_integers() {
    // Signed integer types i8, i16, i32, i64, i128
    let a1: i32 = -12345;
    let a2: i32 = 0xFFF;
    let a3: i32 = 0o177;
    let a4: i32 = 0b1110;

    // Unsigned integer types u8, u16, u32, u64, u128
    let b: u32 = 12345;

    // Architecture specific integer types isize, usize
    let c: isize = 24680;

    println!("\nNumbers are {} {} {} {} {} {}", a1, a2, a3, a4, b, c);
    println!("Numbers in reverse order are {5} {4} {3} {2} {1} {0}", a1, a2, a3, a4, b, c);
    println!("isize is {} bytes on my machine", std::mem::size_of::<isize>())
}
