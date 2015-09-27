
mod exercise1;
mod exercise2;
mod exercise3;

pub fn set1() {
	set1exercise1();
	set1exercise2();
	set1exercise3();
}

use set1::exercise1::{from_hex, to_hex, to_base64};

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
	let actual = to_base64(from_hex(input));
	println!("We got:");
	println!("{}", actual);
	assert_eq!(actual, output);
}

use set1::exercise2::xor_with_key;

fn set1exercise2() {
	let input1 = "1c0111001f010100061a024b53535009181c";
	let input2 = "686974207468652062756c6c277320657965";
	let output = "746865206b696420646f6e277420706c6179";

	println!("Set 1, exercise 2:");
	println!("The HEX string:");
	println!("{}", input1);
	println!("When XORed against:");
	println!("{}", input2);
	println!("Should produce:");
	println!("{}", output);
	println!("We got:");

	let actual = to_hex(xor_with_key(&from_hex(input1), from_hex(input2)));
	println!("{}", actual);
	assert_eq!(actual, output);
}

use set1::exercise3::find_single_xor_key_for_english_text;

fn set1exercise3() {
	let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	println!("Set 1, exercise 3:");
	let output_bin = find_single_xor_key_for_english_text(from_hex(input));
	let result_bin = String::from_utf8(output_bin.1);
	println!("Is {} the key?", output_bin.0);
	println!("{}", result_bin.iter().next().unwrap());

	
}


