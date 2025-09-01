struct Engine{
    horsepower: u32,
}

struct Wheels{
    count: u8,
}

struct Car{
    engine: Engine,
    wheels: Wheels,
    color: String,
}

impl Car {
    fn describe(&self){
        println!("This car has a {} horsepower engine and {} wheels. The color is {}.", self.engine.horsepower,
        self.wheels.count,self.color);
    }
}

fn main() {
    let engine = Engine{ horsepower:500};
    let wheels = Wheels{ count:4};
    let car = Car{
        engine,
        wheels,
        color:String::from("red"),
    };

    car.describe();
}