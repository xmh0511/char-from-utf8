A library that converts utf-8 bytes to a char

````rust
use char_from_utf8::{FromUtf8};
fn main(){
	let r:char = char::from_utf8(&[0xE6,0x88,0x91]).unwrap();
	println!("{r}");

	let r = 0x6211u32.to_utf8().unwrap();
	println!("{r:X?}");
}
````