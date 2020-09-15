global_asm!(include_str!("boot/entry64.asm"));

use crate::libary::{io,sbi};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::libary::interrupt::init();
    unsafe{
        llvm_asm!(
            "ebreak"
            :::
            :"volatile"
        );
    }
    loop {}
}