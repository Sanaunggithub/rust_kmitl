fn main() {
    //mutable slice
    let mut arr = [1,2,3,4,5,6,7,8,9,10];
    let slice_ref = &mut arr[1..3];

    slice_ref[1] = 0;
    println!("Slice: {:?}", slice_ref);

    //ranges in slices

    let arr2 = [11,22,33,44];
    let s1 = &arr2[..2];
    let s2 = &arr2[2..];

    println!("s1: {:?}",s1);
    println!("s2: {:?}",s2);

    //inclusive slices

    let r1 = 2..=3;
    println!("{:?}",&arr[r1]);

}