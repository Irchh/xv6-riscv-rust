# https://github.com/xiaoyang-sde/rust-kernel-riscv/blob/master/Makefile
# LICENSE: https://github.com/xiaoyang-sde/rust-kernel-riscv/blob/master/LICENSE

.PHONY: build fmt doc qemu qemu-gdb gdb clean

build:
	cargo build

fmt:
	cargo fmt

doc:
	cargo doc --no-deps --bin kernel --lib

qemu: build
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/kernel \
    -smp 8

qemu-gdb: build
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/kernel \
    -s -S

gdb:
	riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/debug/kernel' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

clean:
	cargo clean
