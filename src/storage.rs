use std::fmt;

const DEFAULT_CAPACITY: u64 = 10;

pub struct Segment {
	offset: usize,
	bytes: Vec<u8>,
	capacity: u64,
}

impl Segment {
	pub fn new() -> Segment {
		Segment {
			offset: 0,
			bytes: Vec::new(),
			capacity: DEFAULT_CAPACITY,
		}
	}

	pub fn add_byte(&mut self, byte: u8) {
		self.offset += 1;
		self.bytes.push(byte);
	}

	pub fn add_bytes(&mut self, bytes: Vec<u8>) {
		self.bytes.extend(bytes);
	}
}

impl fmt::Display for Segment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} | {}", vec_to_str(&self.bytes), self.offset)
	}
}




fn vec_to_str(vec: &Vec<u8>) -> String {
	vec.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", ")
}
