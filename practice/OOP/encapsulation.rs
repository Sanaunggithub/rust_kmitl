struct User {
    name:String,
    age:u8,
}

impl User{
    fn new(name:String, age:u8) -> Self{
        Self{name,age}
    }

    fn greet(&self){
        println!("Hello, my name is {}",self.name);
    }
}

fn main() {
    let user1 = User{
        name:"San Aung".to_string(),
        age:23,
    };

    user1.greet();
}