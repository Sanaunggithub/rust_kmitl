fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec,bigger_vec];

    for vec in vec_of_vecs {
        let inside = take_fifth(vec);
        if inside.is_some(){
            println!("Found {}",inside.unwrap());
        }

        else {
            println!("Nothing")
        }
    }
}
