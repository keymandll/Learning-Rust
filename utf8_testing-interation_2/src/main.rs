// Refactored and slightly improved version of `utf8_testing-interation_1`.

use std::arch::asm;

struct StringInfo {
    length: usize,
    capacity: usize,
    value: String,
}

fn get_string_byte_at(var: &String, offset: usize) -> u8 {
    let byte: u8;
    unsafe {
        asm!(
            "ldr x0, [{string_addr}]",
            "add x0, x0, {offset}",
            "ldrb {byte:w}, [x0]",
            string_addr = in(reg) var,
            offset = in(reg) offset,
            byte = out(reg) byte,
        );
    }
    byte
}

fn get_string_data(var: &String) -> Vec<u8> {
    let mut value: Vec<u8> = vec!(0; var.len());
    for position in 0..var.len() {
        let byte_value = get_string_byte_at(&var, position);
        value[position] = byte_value;
    }
    value
}

fn get_string_info(s: String) -> StringInfo {
    let v = get_string_data(&s);
    let value = String::from_utf8(v).expect("Found invalid UTF-8");
    StringInfo {
        length: s.len(),
        capacity: s.capacity(),
        value,
    }
}

fn main() {
    let test_value = "💀testing something longer...";
    let test_string = String::from(test_value);
    let string_info = get_string_info(test_string);

    println!("Value: {:?}", string_info.value);
    println!("Length: {}", string_info.length);
    println!("Capacity: {}", string_info.capacity);

    assert_eq!(string_info.value, test_value);
}
