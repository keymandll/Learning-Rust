use std::arch::asm;

fn main() {
    let test_string = String::from("💀test");
    let string_addr_out: u64;
    let data_at_addr: u64;
    let stack_pointer: u64;
    let value: u64;
    let string_len: u64;
    let string_capacity: u64;

    unsafe {
        asm!(
            // Get address of String on stack and put in into
            // `string_addr_out`.
            "mov {stack_pointer}, sp",
            "mov x0, {string_addr}",
            "mov {string_addr_out}, x0",
            // We have String address in `x0`. `x0` supposed to
            // point to the data `ptr`. So we read the data to 
            // see if it is indeed the string `testing`.
            "ldr {data_at_addr}, [x0]",
            "ldr x1, [x0]",
            "ldr {value}, [x1]",
            // Now, I want to see the `len` which is at `x0+8`.
            "add x0, x0, #8",
            "ldr {string_len}, [x0]",
            // And finally, let's check the `capacity`.
            "add x0, x0, #8",
            "ldr {string_capacity}, [x0]",
            stack_pointer = out(reg) stack_pointer,
            string_addr = in(reg) &test_string,
            string_addr_out = out(reg) string_addr_out,
            data_at_addr = out(reg) data_at_addr,
            value = out(reg) value,
            string_len = out(reg) string_len,
            string_capacity = out(reg) string_capacity,
        );
    }

    println!("Stack pointer @ {:#X}", stack_pointer);

    let val = value.to_le_bytes();

    println!("String @ {:#X}", string_addr_out);
    println!("String data @ {:#X}", data_at_addr);
    println!("Value at data ptr: {:#X} ({:?})", value, val);
    println!("String length: {:#X}", string_len);
    println!("String capacity: {:#X}", string_capacity);

}
