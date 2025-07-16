fn main() {
    println!("??");
}

struct User {
    username: String,
    age: u8,
}

impl User {
    fn new(username: String, age: u8) -> Self {
        Self { username, age }
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
