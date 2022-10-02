// Refactored and improved version of `utf8_testing-interation_2`.

fn get_string_data(var: String) -> String {
    let data_ptr = var.as_ptr();
    let v: &[u8];
    unsafe {
        v = core::slice::from_raw_parts(data_ptr, var.len());
    }
    let vec = Vec::from(v);
    String::from_utf8(vec).expect("Found invalid UTF-8")
}

fn main() {
    let test_value = "💀testing something longer...";
    let test_string = String::from(test_value);
    let len = test_string.len();
    let cap = test_string.capacity();
    let read_str = get_string_data(test_string);

    println!("Value: {read_str}");
    println!("Length: {len}");
    println!("Capacity: {cap}");

    assert_eq!(read_str, test_value);
}
