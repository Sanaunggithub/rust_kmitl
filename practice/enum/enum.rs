#[derive(Debug)]
enum Mood {
    Happy,
    Sad,
    Hungry,
}

impl Mood {
    fn check(&self){
        match self {
            Mood::Happy => println!("Happy"),
            Mood::Sad => println!("sad"),
            Mood::Hungry => println!("hungry"),
        }
    }
}

fn main (){
    let my_mood = Mood::Happy;
    my_mood.check();
}
