fn main() {
    // loop {
    // println!("Hello, world!");
    // }
    let mut counter =0;
    let result = loop{
        counter += 1;
        if  counter ==10 {
            break counter *2;
        }
    };
    println!("The result is {result}");

    let mut number =0;
    while number <5{
        println!("{number}!");
        number +=1;
    }
}
