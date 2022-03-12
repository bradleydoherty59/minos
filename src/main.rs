#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Custom testing frameworks because of no_std
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Because of the no_main attribute, we need to provide our own
// entry for the test runner
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
use core::panic::PanicInfo;

// For exiting qemu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where 
    T: Fn(),
{
    fn run(&self){
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop{}
}

// Called during test panics
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    serial_println!("[failed]\n");
    serial_print!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

#[no_mangle] // don't mangle the name of this function
pub extern  "C" fn _start() -> !{
    println!("Hello Brad!");
   
    // Only run if we are in a test context
    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}