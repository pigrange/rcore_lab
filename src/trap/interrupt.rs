use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sscratch, sstatus, stvec,
};

use super::context::TrapFrame;
use crate::println;

use super::timer;
use timer::TICKS as ticks;

global_asm!(include_str!("trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            // 中断处理总入口
            fn __alltraps();
        }

        // 经过上面的分析，由于现在是在内核态
        // 我们要把 sscratch 初始化为 0
        sscratch::write(0);
        // 仍使用 Direct 模式
        // 将中断处理总入口设置为 __alltraps
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);

        // 设置 sstatus 的 SIE 位
        sstatus::set_sie();
    }

    println!("+++++ setup interrupt! +++++")
}

// 删除原来的 trap_handler ，改成 rust_trap
// 以 &mut TrapFrame 作为参数，因此可以知道中断相关信息
// 在这里进行中断分发及处理
#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("undefined trap!!!"),
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer() {
    timer::clock_set_next_event();
    unsafe {
        ticks += 1;
        if ticks == 100 {
            ticks = 0;
            println!("---- 100 ticks ----");
        }
    }
}
