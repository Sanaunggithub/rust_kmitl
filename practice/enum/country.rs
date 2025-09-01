#[derive(Debug)]
struct Country {
    population : u32,
    captial : String,
    leader_name : String,
}

fn main(){
    let population = 5000;
    let captial = String::from("Yangon");
    let leader_name = String::from("Daw Su");

    let result = Country{
        population,
        captial,
        leader_name,
    };

    println!("{:?}",result);
}