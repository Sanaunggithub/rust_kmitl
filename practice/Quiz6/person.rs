struct Person {
    name: String,
    age: u32,
    height: f32,
}

fn main(){
    let person = Person {
        name:String::from("Alice"),
        age: 30,
        height:5.5
    };
    println!("{} {} {}",person.name,person.age,person.height);
}