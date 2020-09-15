OUTPUT_ARCH(riscv)  /*指定架构为riscv*/

ENTRY(_start) /*指定入口点*/

BASE_ADDRESS = 0x80200000; /*定义变量BASE_ADDRESS 表示起始地址*/

SECTIONS 
{
    . = BASE_ADDRESS;/*将当前地址设置为 BASE_ADDRESS*/
    
    start = .; 

    /*代码段*/
    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }
    
    /*rodata 段*/
    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    /*data段*/
    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }
    
    /*stack段*/
    .stack : {
        *(.bss.stack)
    }

    
    /*bss段*/
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }

    PROVIDE(end = .);
}