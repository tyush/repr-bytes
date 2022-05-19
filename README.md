# repr-bytes

Small, simple library to convert byte amounts to
pretty, human readable sizes.

## Quickstart
```rust
let my_file_size = Size::from(54222);

println!("{}", my_file_size); // "54.2 KB"
println!("{}", my_file_size.to_si_string()); // "53.0 KiB"
println!("{}", my_file_size.repr(Units::Bytes)); // "54222 B"
```

## Features
`serde` - enables serialization/deserialization of `Size` <-> usize
