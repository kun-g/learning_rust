#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn inc_signin(&mut self) {
        self.sign_in_count += 1;
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User2 is {:?}", user2);


    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        assert!(user.active);
        user.deactivate();
        assert!(!user.active);
        user.activate();
        assert!(user.active);

        assert_eq!(user.sign_in_count, 1);
        user.inc_signin();
        assert_eq!(user.sign_in_count, 2);
    }
}
