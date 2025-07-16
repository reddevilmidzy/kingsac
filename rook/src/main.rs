fn main() {
	println!("??");
}

fn flag -> bool {
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
