trait Length{
    fn length(&self) -> usize;
}

impl Length for str{
    fn length(&self) -> usize{
        self.chars().count()
    }
}
fn main(){
    let s = "€èe";
    print!("{} {}",s.len(), s.length());
}