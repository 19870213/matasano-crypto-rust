
fn convert_hex2base64(hex_input: &str) -> String {
	return to_base64(from_hex(hex_input));
}

const HEX_0:u8 = ('0' as u8);
const HEX_A:u8 = ('a' as u8);
fn from_hex(hex_input: &str) -> Vec<u8> {
	
	let mut output:Vec<u8> = vec![];
	let mut buffer:u8 = 0;
	let mut count:u8 = 1;
	for c in hex_input.chars() {
		let lower_c = c.to_lowercase().next().unwrap();
		let ic:u8 = ((lower_c as u32) & 0xFF) as u8;
		if '0' <= c && c <= '9' {
			buffer = buffer + (ic - HEX_0);
		}
		if 'a' <= lower_c && lower_c <= 'f' {
			buffer = buffer + (ic - HEX_A) + 10;
		}
		if count == 2 {
			count = 1;
			output.push(buffer);
			buffer = 0;
		} else {
			buffer = buffer << 4;
			count = count + 1;
		}
	}
	if count == 2 {
		output.push(buffer);
	}
	return output;
}
fn to_hex(byte_input: Vec<u8>) -> String {
	static DICTIONARY:[char;16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

	let mut output = String::new();
	for byte in byte_input {
		output.push(DICTIONARY[(byte >> 4) as usize]);
		output.push(DICTIONARY[(byte & 15) as usize]);
	}
	output
}

fn to_base64(byte_input: Vec<u8>) -> String {
	static DICTIONARY:[char;64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
	let mut output:String = String::new();
	for i in 0 .. (byte_input.len()/3) {
		let mut index:usize =  (byte_input[  3*i] as usize) << 16;
		index = index       + ((byte_input[1+3*i] as usize) <<  8);
		index = index       +  (byte_input[2+3*i] as usize);

		output.push(DICTIONARY[index >> 18 & 0x3F]);
		output.push(DICTIONARY[index >> 12 & 0x3F]);
		output.push(DICTIONARY[index >>  6 & 0x3F]);
		output.push(DICTIONARY[index       & 0x3F]);
	}
	let remainder = byte_input.len() % 3;
	let offset:usize = byte_input.len() - remainder;
	if remainder == 2 {
		output.push(DICTIONARY[(byte_input[offset+1] as usize) << 16 + (byte_input[offset+2] as usize) << 8]);
		output.push_str("=");
	} else if remainder == 1 {
		output.push(DICTIONARY[(byte_input[offset+1] as usize) << 16]);
		output.push_str("==");
	}

	return output;
}

fn set1exercise1() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	println!("Set 1, exercise 1:");
	println!("The HEX string:");
	println!("{}", input);
	println!("When converted from and to hex produces:");
	println!("{}", to_hex(from_hex(input)));
	println!("Should produce when converted to Base64:");
	println!("{}", output);
	let actual = convert_hex2base64(input);
	println!("{}", actual);
	assert_eq!(actual, output);
}
fn set1() {
	set1exercise1();
}

fn main() {
	set1();
}
