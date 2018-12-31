pub mod storage;

use crate::storage::Segment;

fn main() {
	let mut seg = Segment::new();
	seg.add_byte(255);
	seg.add_byte(254);
	seg.add_bytes(vec![1, 2, 3, 4]);
    println!("Hello, world! This is my segment: {}", seg);
}
