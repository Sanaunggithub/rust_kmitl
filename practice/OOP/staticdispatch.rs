trait Greet{
    fn greet(&self);
}

struct Dog;
impl Greet for Dog {
    fn greet(&self){
        println!("Wolf WOlf");
    }
}

struct Cat;
impl Greet for Cat{
    fn greet(&self){
        println!("Meow Meow");
    }
}

// Function using generics for static dispatch
fn greet_animal<T:Greet>(animal: T){
    animal.greet();
}
fn main(){
    let my_cat = Cat;
    let my_dog = Dog;

    println!("Using static dispatch with generics:");
    greet_animal(my_dog);
    greet_animal(my_cat);
}