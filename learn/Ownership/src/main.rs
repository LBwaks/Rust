// fn main() {
//     let mut s = String::from("Hello");
//     s.push_str(",world!");
//     let s2 = s; //s goes out scope to prevent double freeing of memeory
//     println!("{ }",s); 
// }

fn main() {
    let s =String::from("Hello");
    takes_ownership(s);
    let x= 5;
    makes_copy(x);

    let s1=gives_ownership();
    let s2=String::from("hello");
    let s3 =takes_and_gives_back(s2);

}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_interger:u32){
    println!("{}",some_interger);

}
//return value and scope
fn gives_ownership() ->String {
    let some_string2 =String::from("yours");
    some_string2
}
fn takes_and_gives_back(a_string:String)->String{
    a_string
}
