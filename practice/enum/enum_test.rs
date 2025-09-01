#[derive(Debug)]
enum Result1<SuccessCode, FailureCode> { //generic over two types: SuccessCode and FailureCode.
    Success(SuccessCode), // variant holds a value of type SuccessCode.
    Failure(FailureCode, char), // variant holds a value of type FailureCode and a char.
    Uncertainty, // variant does not hold any value.
   }

fn main() {
let mut res = Result1::Success::<u32, String>(12u32);
res = Result1::Uncertainty;
res = Result1::Failure("hehe".to_string(), 'd');
println!("{:?}",res);
}