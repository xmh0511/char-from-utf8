pub trait FromUtf8{
	type Output;
	fn from_utf8(code_units:&[u8])->Option<Self::Output>;
}
impl FromUtf8 for char{
	type Output = char;
    fn from_utf8(code_units:&[u8])->Option<Self::Output> {
		if code_units.len() == 1{
			let byte = code_units[0];
			if (byte & 0b10000000) != 0{
				//panic!("invalid single utf-8 code unit");
				return None;
			}
			return char::from_u32(byte as u32);
		}else if code_units.len() > 1{
			let first_byte = code_units[0];
			let bytes = 'bytes_number:{
				for i in 2..=7{
					let i = 9 - i;
					let test = 2u8.pow(i);
					if (first_byte & test) == 0{
						break 'bytes_number 7 - i;
					}
				}
				//panic!("invalid utf-8 code units sequence");
				return None;
			};
			if bytes > code_units.len() as u32{
				//panic!("invalid utf-8 code units sequence: expected {bytes} bytes but only has {}",code_units.len());
				return None;
			}
			//let high_byte = (2u8.pow(bytes) - 1) & first_byte;
			let mut code_point = 0u32;
			let last_low_byte_index = bytes - 1;
			for k in 1..=last_low_byte_index{
				let index = bytes - k;
				let code_unit = code_units[index as usize];
				code_point = code_point | ((code_unit as u32 & 0b00111111) << ((k - 1)*6));
			}
			let high_byte = ((2u8.pow(bytes) - 1) & first_byte) as u32;
			let high_byte = high_byte << (last_low_byte_index*6);
			code_point = code_point | high_byte;
			return char::from_u32(code_point);
		}else{
			//panic!("invalid utf-8 code units sequence");
			return None;
		}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
		assert_eq!(char::from_utf8(&[0x61]), Some('a'));
		assert_eq!(char::from_utf8(&[0xC3,0x80]), Some('À'));
        assert_eq!(char::from_utf8(&[0xE6,0x88,0x91]), Some('我'));
		assert_eq!(char::from_utf8(&[0xF0,0x93,0x83,0xB0]), Some('𓃰'));
    }
}
