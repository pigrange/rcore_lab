use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)] //表示按照C语言的标准进行内存布局
pub struct TrapFrame {
    pub x :[usize;32], // 32个通用寄存器数组
    pub sstatus :Sstatus, //Supervisor Status 寄存器
    pub sepc :usize, // Supervisor expection program counter
    pub stval : usize, // Supervisor trap value
    pub scause : Scause, // Scause register: record the cause of exception/interrupt/trap
}