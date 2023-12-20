# "cargo run" has been configured to run bootloader (.cargo/config.toml)
# so alternatively, you can simply run by "cargo run" command

qemu-system-x86_64 -drive format=raw,file=target/x86_64_target/debug/bootimage-operating-system-from-scratch.bin