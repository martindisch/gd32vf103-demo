# gd32vf103-demo

A small example for running Rust code on RISC-V.

The specific hardware used is:
* GD32VF103CBT6 on a custom board
* Segger J-Link EDU Mini debugger for flashing

## Installation

First of all, you need to install the `rust-std` components (pre-compiled
`core` crate) for RISC-V.
```console
$ rustup target add riscv32imac-unknown-none-elf
```

That covers Rust, but you also need the RISC-V GNU compiler toolchain,
specifically the `riscv64-unknown-elf-gdb` executable. There may already be a
package for your distribution, but for me on Arch Linux that was not the case
(except for an outdated AUR entry). You may need to compile it yourself [from
source](https://github.com/riscv/riscv-gnu-toolchain). For the installation,
follow the instructions from the [Newlib
section](https://github.com/riscv/riscv-gnu-toolchain#installation-newlib).

Finally, if you're using the J-Link EDU Mini like me, you need to get the
[J-Link Software and Documentation
Pack](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack)
for that. Extract it somewhere, copy the `99-jlink.rules` file to
`/etc/udev/rules.d/` and reload the rules with
```console
$ sudo udevadm control --reload-rules
```
If you hook up your board to the J-Link and power it, you should be able to
verify that it works with
```console
$ ./JLinkExe -device GD32VF103CBT6 -if JTAG -speed 4000 -JTAGConf -1,-1
J-Link> connect
J-Link> q
```

## Building
From the directory with the J-Link software, start the GDB server.
```console
$ ./JLinkGDBServer -device GD32VF103CBT6 -if JTAG -speed 4000 -port 3333
```
In the project directory, build, flash and enter GDB with
```console
$ cargo run
```
This will break at the main function, so you need to
```console
(gdb) continue
```

## License
Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
