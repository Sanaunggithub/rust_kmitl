#[derive(Debug)]
struct Animal {
    age : u32,
    animal_type : AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Dog,
    Cat,
}

impl Animal {
    fn new() -> Self {
        Self {
            age : 10,
            animal_type : AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("Changing to dog");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        println!("Changing to cat");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        use AnimalType::*;

        match self.animal_type {
            Dog => println!("It's dog"),
            Cat => println!("It's cat"),
        }
    
    }

}

fn main() {
    let mut my_animal = Animal::new();
    println!("{:?}" ,my_animal);
    my_animal.check_type();
    my_animal.change_to_dog();
    my_animal.check_type();
    
}