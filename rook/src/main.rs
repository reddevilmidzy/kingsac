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
}
struct User {
    username: String,
    age: u16,
}

impl User {
    fn new(username: String, age: u16) -> Self {
        Self { username, age }
    }

    fn is_adult(&self) -> bool {
        self.age >= 19
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_age(&self) -> u16 {
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
        std::thread::sleep(std::time::Duration::from_secs(1));
        let user = User::new("Redddy".to_string(), 20);
        assert_eq!(user.get_username(), "Redddy");
        assert_eq!(user.get_age(), 20);
    }

    #[test]
    fn test_user_is_adult() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let user = User::new("Redddy".to_string(), 20);
        assert_eq!(user.is_adult(), true);
    }

    #[test]
    fn test_team_new() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let team = Team::new(
            "사자보이즈".to_string(),
            vec![
                User::new("진우".to_string(), 400),
                User::new("애비".to_string(), 20),
            ],
        );
        assert_eq!(team.name, "사자보이즈");
        assert_eq!(team.members.len(), 2);
    }
}
