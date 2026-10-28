## VGA Testing

> This project is meant to demostrate how the `VGA` works in a ***'bare-bones'*** Operating System.

## How To Run

Make sure you have these things installed:

- **QEMU**: For a virtual machine
- **Rust**: The Standard compiler and Nightly
- **bootimage**: Because we want to create a `.bin` file later

1. Clone the repository.
2. Run `cargo bootimage` to create a `.bin`, we'll use this for our virtual machine
3. Once finished compiling, jump to `target/x86_64-kernel/debug`.
4. Then once you see `bootimage-vga_testing.bin`, run this command:
```bash
qemu-system-x86_64 -drive format=raw,file=bootimage-vga_testing.bin
```

You should see: ***'Welcome! Hello, World!'*** printed on the virtual machine.

## File Structure

```txt
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   └── vga
│       ├── color.rs
│       ├── mod.rs
│       └── writer.rs
└── x86_64-kernel.json
```

## License

This project is licensed under the [MIT License](./LICENSE).