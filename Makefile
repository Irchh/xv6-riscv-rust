# https://github.com/xiaoyang-sde/rust-kernel-riscv/blob/master/Makefile
# MIT License
#
# Copyright (c) 2023 Xiaoyang Liu
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

.PHONY: build fmt doc qemu qemu-gdb gdb clean

build:
	cargo build

build-release:
	cargo build --release

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

qemu-release: build-release
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv64gc-unknown-none-elf/release/kernel \
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
