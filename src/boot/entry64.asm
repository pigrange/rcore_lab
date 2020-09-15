.section .text.entry //在text段一定一个.entry
.globl _start

_start:
    la sp,bootstacktop
    call rust_main

    .section .bss.stack
    .align 12
    .globl bootstack

//分配16KB的启动栈
bootstack:
    .space 4096 * 4
    .globl bootstacktop

bootstacktop:
