target extended-remote :3333
set print asm-demangle on
set backtrace limit 32
break panic
monitor arm semihosting enable
