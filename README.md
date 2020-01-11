# Simput
Simput enables an easy, simple method to get user inputs.
__This is not intended to use in production environment.__

## Installation
In `Cargo.toml`:
```toml
[dependencies]
simput = "0.1"
```

## Usage
`input!` macro parses input from standard input as a tuple, which contains values of types specified by parameters.
Each values are split by ascii space (0x20).
```rust
use simput::input;
let (number, word) = input!(i32, String);

// stdin: 16 Hello
assert_eq!(16, number);
assert_eq!("Hello", word);
```
You can use `Line` keyword to read a whole line.
In this case, a `String` is returned.
```rust
let i_am_a_line = input!(Line);

// stdin: The quick brown fox jumps over the lazy dog
assert_eq!("The quick brown fox jumps over the lazy dog", i_am_a_line);
```