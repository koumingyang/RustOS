//! Entrance and initialization for aarch64.
extern crate atags;

pub mod io;
pub mod paging;
pub mod memory;
pub mod interrupt;

#[cfg(feature = "board_raspi3")]
#[path = "board/raspi3/mod.rs"]
pub mod board;

/// TODO
/// The entry point of kernel
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    // Init board to enable serial port.
    board::init();

    // First init log mod, so that we can print log info.
    // FIXME
    // ::logging::init();

    let (start, end) = memory::memory_map().expect("failed to find memory map");
    println!("The value of start is: {}, end is {}", start, end);

    memory::init();
    println!("memory init over");

    let mut v = vec![];
    for i in 0..1000 {
        v.push(i);
        println!("{:?}", v);
    }

    super::fs::show_logo();

    loop {
        print!(">> ");
        loop {
            let c = io::getchar();
            match c {
                '\u{7f}' => {
                    print!("\u{7f}");
                }
                ' '...'\u{7e}' => {
                    print!("{}", c);
                }
                '\n' | '\r' => {
                    print!("\n");
                    break;
                }
                _ => {}
            }
        }
    }

    // ::kmain();
}

global_asm!(include_str!("boot/boot.S"));
