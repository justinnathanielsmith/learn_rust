/// Exercise 4.1: User struct
/// It should have the following fields:
/// - `name`: String
/// - `email`: String
/// - `age`: u8
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    /// Exercise 4.2: Implement a `new` associated function (constructor)
    /// It should take `name: String`, `email: String`, and `age: u8` 
    /// and return a new `User` instance.
    pub fn new(name: String, email: String, age: u8) -> Self {
        todo!("Implement the User::new function")
    }

    /// Exercise 4.3: Implement a `change_email` method
    /// It should take a mutable reference to self and a new email string (&str),
    /// and update the `email` field of the user.
    pub fn change_email(&mut self, new_email: &str) {
        todo!("Implement the change_email method")
    }
}

/// Exercise 4.4: Rectangle struct
/// It should have `width` and `height` fields, both of type `u32`.
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Exercise 4.5: Implement an `area` method
    /// It should return the area of the rectangle (width * height).
    pub fn area(&self) -> u32 {
        todo!("Implement the area method")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let name = String::from("Alice");
        let email = String::from("alice@example.com");
        let age = 30;
        let user = User::new(name.clone(), email.clone(), age);
        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
        assert_eq!(user.age, age);
    }

    #[test]
    fn test_user_change_email() {
        let mut user = User {
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
            age: 30,
        };
        let new_email = "alice_new@example.com";
        user.change_email(new_email);
        assert_eq!(user.email, new_email);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 10, height: 20 };
        assert_eq!(rect.area(), 200);
    }
}
