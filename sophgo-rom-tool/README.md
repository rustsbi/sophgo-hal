# 算能芯片 ROM 辅助工具

本工具可由二进制文件生成算能芯片 ROM 镜像文件（fip.bin）。

Example:

```bash
cargo build -p hello-world --target riscv64imac-unknown-none-elf --release
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary .\target\riscv64imac-unknown-none-elf\release\hello-world .\target\hello-world.bin
cargo run --bin sophgo-rom-tool -- .\target\hello-world.bin -o .\target\fip.bin
```
