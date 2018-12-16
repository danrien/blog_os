#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use blog_os::{exit_qemu, serial_println};
use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::interrupts::init_idt();

    // invoke a breakpoint exception
    x86_64::instructions::int3();

    serial_println!("failed");
    serial_println!("Expected to panic!");

    unsafe {
        exit_qemu();
    }

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    serial_println!("ok");

    unsafe {
        exit_qemu();
    }

    loop {}
}
