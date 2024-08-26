# roblox-rs

roblox-rs is a Rust to Luau transpiler, designed to allow developers to write Rust code that can be run on the Roblox platform.

## Current Features

- Basic function transpilation
- Variable declarations and assignments
- Type mapping from Rust to Luau
- Control flow structures:
  - If-else statements
  - For loops (with range-based iteration)
  - While loops
  - Infinite loops
- Match expressions (basic support)
- Binary operations

## Usage

```bash
cargo run -- -f ./sample.rs
```

## Roadmap

- [ ] Improve match expression support
  - [ ] Handle more complex patterns
  - [ ] Support guard clauses

- [ ] Implement struct and enum transpilation
  - [ ] Convert Rust structs to Luau tables
  - [ ] Handle enum variants

- [ ] Add support for more Rust expressions
  - [ ] Array and slice operations
  - [ ] Closures
  - [ ] Method calls

- [ ] Implement error handling
  - [ ] Convert Rust's Result and Option types to Luau equivalents
  - [ ] Proper error reporting during transpilation

- [ ] Support for Rust standard library functions
  - [ ] Implement Luau equivalents for common Rust std functions

- [ ] Module system
  - [ ] Handle Rust modules and convert them to Luau modules

- [ ] Roblox-specific features
  - [ ] Integration with Roblox APIs
  - [ ] Support for Roblox-specific types and functions

- [ ] Optimization passes
  - [ ] Improve generated Luau code efficiency

- [ ] Documentation and examples
  - [ ] Comprehensive documentation on usage and limitations
  - [ ] Example projects showcasing the transpiler's capabilities

- [ ] Testing framework
  - [ ] Unit tests for transpiler components
  - [ ] Integration tests with sample Rust projects

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
