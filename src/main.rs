#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

static HELLO: &[u8] = b"Hello World! My name is Brad";

#[no_mangle] // don't mangle the name of this function
pub extern  "C" fn _start() -> !{
    // Cast VGA buffer address into a raw pointer
   let vga_buffer = 0xb8000 as *mut u8;

   // Iterate over the bytes of the static Hello string
   // Using enumerate to get additional running var i
   for (i, &byte) in HELLO.iter().enumerate() {
       // Unsafe because raw pointers are not provably valid
       // but we know this is where the VGA buffer is 
        unsafe {
                // Offset method allows writing string byte from
                // HELLO
                *vga_buffer.offset(i as isize * 2) = byte;
                // Use cyan (0xb) as color for VGA
                *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            }
   }

   loop {}
}
