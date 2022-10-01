// Ignore the unused function warning in `cargo run` output. Obviously
// that's not the point.

use std::arch::asm;

fn corrupt_utf8_str_data(var: &String) {
    // Simple code to corrupt the string data as part of the simulation.
    unsafe {
        asm!(
            "ldr x0, [{string_addr}]",
            "strb w0, [x0, #10]",
            string_addr = in(reg) var,
        );
    }

}

fn get_string_data_bad(var: String) -> String {
    let data_ptr = var.as_ptr();
    let v: &[u8];
    unsafe {
        v = core::slice::from_raw_parts(data_ptr, var.len());
    }
    let vec = Vec::from(v);

    // Yeah, really should not, but...
    String::from_utf8(vec).unwrap()
}


fn main() {
    let test_value = "💀 Yup, appropriate emoji.";
    let test_string = String::from(test_value);

    // Here we *simulate* a more complex app that has e.g. a vulnerability
    // somewhere that allowed someone to corrupt the string data on the 
    // heap between us defining the String above and fetching the string
    // data later.
    corrupt_utf8_str_data(&test_string);

    let value = get_string_data_bad(test_string);
    println!("Value read: {value}"); // Notice the word `@ppropriate`

    // The two values are not the same, but, without this assert we'd
    // have to manually look to see.
    assert_eq!(test_value, value);
}
