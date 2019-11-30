target extended-remote :3333

set confirm off
set remotetimeout 240

set print asm-demangle on

break main

load
continue
