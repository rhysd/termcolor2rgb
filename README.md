`termcolor2rgb`
===============

[termcolor2rgb][crate] is a small crate to extend [termcolor](https://crates.io/crates/termcolor) crate.
It provides [`to_rgb()`]() method to [`termcolor::Color`](https://docs.rs/termcolor/1.0.4/termcolor/enum.Color.html) instance.

```rust
use termcolor;

// Declare extending termcolor::Color by using a trait.
// .to_rgb() method is made available.
use termcolor2rgb::ColorExt;

// Basic 8 colors
assert_eq!(termcolor::Color::Red.to_rgb(), (0x80, 0, 0));

// ANSI 256 colors
assert_eq!(termcolor::Color::Ansi256(223).to_rgb(), (0xff, 0xd7, 0xaf));

// RGB color
assert_eq!(termcolor::Color::Rgb(0x12, 0x89, 0xef).to_rgb(), (0x12, 0x89, 0xef));
```

## Installation

Add dependencies to your `Cargo.toml`.

```
termcolor = "1.0"
termcolor2rgb = "1.0"
```

## License

[The MIT License](LICENSE.txt)

[crate]: https://crates.io/crates/termcolor2rgb
