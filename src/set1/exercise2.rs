
pub fn xor_with_key(input:&Vec<u8>, key:Vec<u8>) -> Vec<u8> {
	let mut output:Vec<u8> = Vec::with_capacity(input.len());
	let mut key_i:usize = 0;
	for input_i in 0..input.len() {
		output.push(input[input_i] ^ key[key_i]);
		key_i = (key_i + 1) % key.len();
	}

	output
}

