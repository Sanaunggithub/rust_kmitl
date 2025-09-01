trait Diet {
    fn food(&self) -> String;
}

trait Habitat{
    fn environment(&self) -> String;
}

struct Duck;
struct Chicken;

impl Diet for Duck{
    fn food(&self)->String{
        return "Duck eats plants and small fish".to_string()
    }
}

impl Diet for Chicken{
    fn food(&self)->String{
        return "Chicken eats seeds and insects".to_string()
    }
}

impl Habitat for Duck{
    fn environment(&self) -> String {
        return "Duck lives in weltands and ponds".to_string()
    }
}

impl Habitat for Chicken{
    fn environment(&self)-> String{
        return "Chicken lives in farms and backyard".to_string()
    }
}

fn describe_animal<T: Diet+ Habitat>(animal:&T) {
    println!("{}",animal.food());
    println!("{}",animal.environment());
}

fn main() {
    let my_duck = Duck;
    let my_chicken = Chicken;
    describe_animal(&my_duck);
    describe_animal(&my_chicken);
}