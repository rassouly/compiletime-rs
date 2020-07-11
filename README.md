# Get the time at compile-time!

Currently required nightly Rust due to [rust#54727](https://github.com/rust-lang/rust/issues/54727)

Example: 
```rs
fn main() {
	println!("This program was compiled {} ms after 1 January 1970!", compiletime::milliseconds!());
}
```