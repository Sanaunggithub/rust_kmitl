enum ThingsInTheSky {
    Stars(String),
    Sun(String),
}

fn create_skystate (time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun")),
        _ => ThingsInTheSky::Stars(String::from("I can see the star")),
    }
}

fn check_skystate (state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{}",description),
        ThingsInTheSky::Stars(description) => println!("{}",description),
    }
}

fn main() {
    let time = 8;
    let sky_state = create_skystate(time);   
    check_skystate(&sky_state);
}