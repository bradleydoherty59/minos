#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
// Custom testing frameworks because of no_std
#![feature(custom_test_frameworks)]
#![test_runner(minos::test_runner)]
// Because of the no_main attribute, we need to provide our own
// entry for the test runner
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minos::println;

#[no_mangle] // don't mangle the name of this function
pub extern  "C" fn _start() -> ! {
    println!("Hello Brad!");
   
    // Only run if we are in a test context
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minos::test_panic_handler(info)
}