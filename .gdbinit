target remote :3333
set print asm-demangle on
load
break sos::main
continue
