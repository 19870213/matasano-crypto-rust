
use set1::exercise2::xor_with_key;

pub fn find_single_xor_key_for_english_text(input:Vec<u8>) -> (u8, Vec<u8>) {
	for key in (0..256).map(|x| x as u8) {
		let xored = xor_with_key(&input, vec![key]);
		if all_english_characters(&xored) {
			return (key, xored);
		}
	}
	(0, input)
}

fn all_english_characters(input:&Vec<u8>) -> bool {
	for b in input {
		if *b < (' ' as u8) {
			return false;
		}
		if *b > 127 {
			return false;
		}
		if *b == ('[' as u8) || *b == (']' as u8) {
			return false;
		}
		if *b == ('{' as u8) || *b == ('}' as u8) {
			return false;
		}
		if *b == ('(' as u8) || *b == (')' as u8) {
			return false;
		}
		if *b == ('*' as u8) || *b == ('-' as u8) {
			return false;
		}
		if *b == ('+' as u8) || *b == ('/' as u8) {
			return false;
		}


	}
	true
}
