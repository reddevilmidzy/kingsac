fn main() {
    println!("??");
}

struct Team {
    name: String,
    members: Vec<User>,
}

impl Team {
    fn new(name: String, members: Vec<User>) -> Self {
        Self { name, members }
    }

    fn print() {
        println!("hello");
    }
}

struct User {
    username: String,
    age: u8,
}

impl User {
    fn new(username: String, age: u8) -> Self {
        Self { username, age }
    }

    fn is_adult(&self) -> bool {
        self.age >= 19
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

    #[test]
    fn test_user_new() {
        let user = User::new("Redddy".to_string(), 20);
        assert_eq!(user.get_username(), "Redddy");
        assert_eq!(user.get_age(), 20);
    }

    #[test]
    fn test_user_is_adult() {
        let user = User::new("Redddy".to_string(), 20);
        assert_eq!(user.is_adult(), true);
    }
}
