
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
    pub active: bool,
}


impl User {
    pub fn user_details(&self) {
        println!("Name: {}, Email: {}, Age: {}", self.name, self.email, self.age);
    }
}