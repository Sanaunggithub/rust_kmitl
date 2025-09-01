fn main(){
    let i = (1..20).step_by(2);
    let x = (1..7).rev().step_by(2);
   
    println!("{:?} ",i);
    println!("{:?} ",x);
  

    
    let i: Vec<_> = (1..20).step_by(2).collect();
    let x: Vec<_> = (1..7).rev().step_by(2).collect();
    let y: Vec<_> = (-1..-10).step_by(1).collect();
    
    println!("{:?}", i);
    println!("{:?}", x);
    println!("{:?}", y);
    
    
}