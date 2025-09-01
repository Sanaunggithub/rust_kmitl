fn outer(){
    fn inner(){
        println!("Inside");
    }
    inner();     
    println!("Outside");

}

fn main(){
    outer();
}