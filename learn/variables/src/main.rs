use std::io;
// fn main() {
//     //variables
//     let mut x= 5;
//     println!("The value of x is {x}");
//      x=6 ;
//     println!("The value of x is {x}");
//     //data types
//     // tuples

//     let tup: (i32,u32,f32) =(500,344,2.5);
//     let (_x,_y,_z)=tup;
//     let one = tup.1;
//     println! ("Tup is {one}")

// }

fn main(){
    let a = [1,2,3,4,5];
    println!("Please enter an array index");
    let mut index=String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index:usize =index.trim().parse().expect("
    Index entered was not a number");
    let element = a[index];
    println!("The value of the elemrnt at index{index} is:{element}");

}
