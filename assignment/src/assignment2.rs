fn sum(vector: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vector {
        sum = sum + item
    }
    sum
}
fn main() {
    let v = vec![1, 2, 3];
    let s = sum(&v);
    println!("sum of {:?}: {}", &v, s);
}

// fn sum(left: i32, right: i32) -> i32 {
//     left + right
// }
// fn main() {
//     let a = 42;
//     let b = 1;
//     let s = sum(a, b);
//     println!("this sum of {} and {} is {}", a, b, s);
// } // no error!
