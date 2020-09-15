global_asm!(include_str!("boot/entry64.asm"));

use crate::trap::interrupt;
use crate::trap::timer;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    timer::init();
    unsafe{
        llvm_asm!(
            "ebreak"
            :::
            :"volatile"
        );
    }
    loop {}
}