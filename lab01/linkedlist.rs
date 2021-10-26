fn main() {
	let mut llist = LLNode {
		value: 0
		next: None;
	};

	println!("LIST: {:?}", llist);
}

// Definition of the Linked List Node
pub struct LLNode {
	pub value: u64;
	pub next: Option<&Box<LLNode>>
}