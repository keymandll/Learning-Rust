// Refactored and slightly extended version of the `memory_check`.

use std::arch::asm;

enum StringProperty {
    LENGTH,
    CAPACITY,
}

struct StringInfo {
    address: u64,
    data: u64,
    length: u64,
    capacity: u64,
    value: String,
}

fn raw_value_to_string(raw_value: u64) -> String {
    let vec = Vec::from(raw_value.to_le_bytes());
    String::from_utf8(vec).expect("Found invalid UTF-8")
}

fn get_string_address(var: &String) -> u64 {
    let string_addr_out: u64;

    unsafe {
        asm!(
            "mov x0, {string_addr}",
            "mov {string_addr_out}, x0",
            string_addr = in(reg) var,
            string_addr_out = out(reg) string_addr_out,
        );
    }
    string_addr_out
}

fn get_string_data(string_address: u64) -> u64 {
    let bytes: u64;

    unsafe {
        asm!(
            "mov x0, {string_addr}",
            "ldr x1, [x0]",
            "ldr {bytes}, [x1]",
            string_addr = in(reg) string_address,
            bytes = out(reg) bytes,
        );
    }
    bytes
}

fn get_string_property(string_address: u64, property: StringProperty) -> u64 {
    let value: u64;
    let offset: u64 = match property {
        StringProperty::LENGTH => { 8 },
        StringProperty::CAPACITY => { 16 },
    };

    unsafe {
        asm!(
            "mov x0, {string_addr}",
            "mov x1, {offset}",
            "add x0, x0, x1",
            "ldr {value}, [x0]",
            string_addr = in(reg) string_address,
            offset = in(reg) offset,
            value = out(reg) value,
        );
    }
    value
}

fn get_string_length(string_address: u64) -> u64 {
    get_string_property(string_address, StringProperty::LENGTH)
}

fn get_string_capacity(string_address: u64) -> u64 {
    get_string_property(string_address, StringProperty::CAPACITY)
}

fn get_string_info(s: String) -> StringInfo {
    let address = get_string_address(&s);
    let data = get_string_data(address);
    let value = raw_value_to_string(data);

    StringInfo {
        address,
        data,
        length: get_string_length(address),
        capacity: get_string_capacity(address),
        value,
    }
}

fn main() {
    let test_value = "💀test";
    let test_string = String::from(test_value);
    let string_info = get_string_info(test_string);

    println!("String @ {:#X}", string_info.address);
    println!("  Value: {:#X} ({:?})", string_info.data, string_info.value);
    println!("  Length: {}", string_info.length);
    println!("  Capacity: {}", string_info.capacity);

    assert_eq!(string_info.value, test_value);
}
