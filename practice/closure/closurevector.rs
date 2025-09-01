fn main() {
    print!("{}", (|v:Vec<i32>|{
        let mut sum = 0;
        for i in 0..v.len() {
            sum += v[i];
        }
        sum
    })(vec![1,2,3,4,5]));
}