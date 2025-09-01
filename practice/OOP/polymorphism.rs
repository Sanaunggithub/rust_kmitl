trait Greet {
    fn greet(&self);
}

struct Dog;
impl Greet for Dog {
    fn  greet(&self){
        println!("WOlf WOLF");
    }    
}

struct Cat;
impl Greet for Cat{
    fn greet(&self){
        println!("MEOW MEOW");
    }
}

fn greet_animal(animal:&dyn Greet){
    animal.greet();
}

fn main() {
    let my_cat = Cat;
    let my_dog = Dog;

    println!("Using trait");
    greet_animal(&my_dog);
    greet_animal(&my_cat);

    println!("Using struct impl");
    my_cat.greet();
    my_dog.greet();

}