trait Container{
    type Item;

    fn add(&mut self, item:Self::Item);
    fn get(&self)-> Option<&Self::Item>;
}
struct VecContainer<T>{
    items:Vec<T>
}

impl <T>Container for VecContainer<T>{
    type Item = T;

    fn add(&mut self,item: Self::Item){
        self.items.push(item)
    }

    fn get(&self)-> Option<&Self::Item>{
        self.items.last()
    }
}

fn main(){
    let mut vec_container = VecContainer{ items: Vec::new()};
    
    for i in 0..5{
        vec_container.add(i);
    }
    println!("The added item is {:?}",vec_container.get().unwrap());
}