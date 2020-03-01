target extended-remote :3333

set confirm off
set remotetimeout 240

set print asm-demangle on

break DefaultHandler
break HardFault
break rust_begin_unwind

break main

load
continue
