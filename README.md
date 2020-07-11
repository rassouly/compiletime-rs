# Get the time at compile-time!

Example: 
```rs
fn main() {
	println!("This program was compiled {} ms after 1 January 1970!", compiletime::milliseconds!());
}
```