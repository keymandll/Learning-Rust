use std::arch::asm;

// const STRING_PROPERTY_CAPACITY: usize = 8;
const STRING_PROPERTY_LENGTH: usize = 16;

fn update_string_prop_on_stack(var: &String, property: usize, value: usize) {
    // Simple code to corrupt the string properties on the stack.
    unsafe {
        asm!(
            "mov x0, {string_addr}",
            "add x0, x0, {property}",
            "str {value}, [x0]",
            string_addr = in(reg) var,
            property = in(reg) property,
            value = in(reg) value,
        );
    }
}

fn print_string_info(s: &String) {
    println!("  Value: {}", s);
    let len = s.len();
    println!("  Bytes count (len): {}", len);
    let chars_count = s.chars().count();
    println!("  Number of characters: {}", chars_count);
}

fn main() {
    let test_value = "💀 Yup, appropriate emoji.";
    // ^^^ 25 characters
    //     24 + 4 bytes (The utf-8 skull is 4 bytes)

    // ------------------------------------------------------------------------
    // [1] Print unmodified string details 
    // ------------------------------------------------------------------------

    println!("[1] Original string:");
    let test_string = String::from(test_value);
    print_string_info(&test_string);

    // ------------------------------------------------------------------------
    // [2] Modify string length on stack 
    // ------------------------------------------------------------------------

    let new_length: usize = 512000000;
    // As we will see, the string data is not being read until a terminating
    // NULL byte, but `length` bytes are read from the heap.
    // As mentioned at https://doc.rust-lang.org/rust-by-example/std/str.html
    // Strings are NOT null terminated.
    println!("[2] Updating string length to {} ({:#X}).", new_length, new_length);
    update_string_prop_on_stack(&test_string, STRING_PROPERTY_LENGTH, new_length);

    // ------------------------------------------------------------------------
    // [3] Print unmodified string details 
    // ------------------------------------------------------------------------

    println!("[3] New string:");
    print_string_info(&test_string);

    // ------------------------------------------------------------------------
    // [4] Clone string
    // ------------------------------------------------------------------------

    println!("[4] Cloned string:");
    let cloned_string = test_string.clone();
    print_string_info(&cloned_string);

    // ------------------------------------------------------------------------
    // [5] String -> array of u8
    // ------------------------------------------------------------------------

    // println!("[5] As bytes:");
    // let bytes: &[u8] = test_string.as_bytes();
    // Better not print it below as it's way too long. Some junk from memory
    // should be visible printed earlier.
    // println!("  Byte values: {:?}", bytes);

}
