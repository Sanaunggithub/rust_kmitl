struct Book {
    number : u32,
}

impl Book {
    fn get_number(&mut self) -> u32 {
        self.number
    }

    fn change_number(&mut self, new_number: u32){
        self.number = new_number;
    }

    fn new (number: u32) -> Self {
        Self {
            number
        }  
    }
}

fn main() {
    let mut my_book = Book::new(170);

    //my_book.change_number(100);
    println!("{:?}", my_book.get_number());
}