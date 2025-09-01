fn main(){
    let name:String = String::from("TutorialsPoint");
    display(name); 
    //cannot access name after display
    //if you want to access againg,use & reference
 }
 fn display(param_name:&String){
    println!("param_name value is :{}",param_name);
 }