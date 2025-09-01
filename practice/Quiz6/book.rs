struct Book {
    title: String,
    author: String,
    published_year: Option<u32>
}

fn main() {
    let my_book = Book {
        title:String::from("War and Peace"),
        author:String::from("Leo Tolstoy"),
        published_year:Some(1867)
    };
    match my_book.published_year {
        Some(year) => println!("{} was published in {}", my_book.title, year),
        None => println!("{} has no publication year", my_book.title),
    }

}