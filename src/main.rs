mod shared;
use shared::ops::multiply; 

fn main() {
    println!("Hello github actions!");
    let test = multiply(3,5);
    let _print_msg = format!("{test}");
    println!("Hello {test}!");
}   
