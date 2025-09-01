mod my_module {
    pub fn public_function(){
        let local = "accessible within this function";
        println!("{}",local);
    }
    fn private_function(){
        println!("Private function");
    }
}

fn main(){
    my_module::public_function();
   // my_module::private_function();
}