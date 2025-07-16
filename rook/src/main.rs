fn main() {
	println!("??");
}

struct User {
	name: String,
	age: u32,
}

fn flag() -> bool {
	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_flag() {
		assert!(flag());
	}
}
