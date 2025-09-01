fn main(){
    let state_code = "MH";
    let state = match state_code {
       "MH" => {println!("Found match for MH"); "hehe" /*this is return value you cannot remove it.Also the type must be consistent */},
       "KL" => "Kerala",
       "KA" => "Karnadaka",
       "GA" => "Goa",
       _ => "Unknown"
    };
    println!("State name is {}",state);
}