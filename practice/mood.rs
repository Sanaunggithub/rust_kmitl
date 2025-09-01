enum Mood {
    Happy,
    Sad,
    Angry,
}

fn match_mood (mood : &Mood) -> i32 {
    use Mood::*;
    match mood {
        Happy => 10,
        Sad => 3,
        Angry => 1,  
    }
}

fn main() {
    let my_mood = Mood::Happy;
    let level = match_mood(&my_mood);
    println!("My Happiness level is {}",level);
}