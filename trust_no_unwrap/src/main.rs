// Ignore the unused function warning in `cargo run` output. Obviously
// that's not the point.

use std::{arch::asm, string::FromUtf8Error};

fn corrupt_utf8_str_data(var: &String) {
    // Simple code to mess up our pretty utf-8 data as part of the simulation.
    unsafe {
        asm!(
            "ldr x0, [{string_addr}]",
            "add x1, x0, #2",
            "str x0, [x1]",
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

    // Unwrap is good as long as we can ensure data integrity,
    // but for now it seems to me it's always better to go with
    // what is implemented by `get_string_data_good`.
    String::from_utf8(vec).unwrap()
    // Funny thing is that I got to this conclusion with ever 
    // looking at `unwraps` documentation that clearly says 
    // what I just said before. I'm so proud of myself. :D
}

fn get_string_data_good(var: String) -> Result<String, FromUtf8Error> {
    let data_ptr = var.as_ptr();
    let v: &[u8];
    unsafe {
        v = core::slice::from_raw_parts(data_ptr, var.len());
    }
    let vec = Vec::from(v);
    // I believe probably best to do this to be safe. Then, handle
    // Result in the caller.
    String::from_utf8(vec)
}

fn main() {
    let test_value = "💀 Yup, appropriate emoji.";
    let test_string = String::from(test_value);

    // Here we *simulate* a more complex app that has e.g. a vulnerability
    // somewhere that allowed someone to corrupt the string data on the 
    // heap between us defining the String above and fetching the string
    // data later using `get_string_data`.
    corrupt_utf8_str_data(&test_string);

    // [1] Try with this first to see what happens.
    let value = get_string_data_bad(test_string);
    println!("Value: {value}");

    // [2] Then, comment the above two lines, uncomment the below and see
    // what happens.

    /*
    match get_string_data_good(test_string) {
        Ok(res) => println!("Value: {res}"),
        Err(_) => println!("Sorry, things got corrupted."),
    }
    */
}
