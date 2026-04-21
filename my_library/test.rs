
use std::io;
fn input()->i32{
    let mut inp = String:: new();
    io::stdin().read_line(&mut inp).unwrap();
    let num:i32 = inp.trim().parse().unwrap();
    return num;
}

// fn input() -> i32 {
//     let mut inp = String::new();
//     io::stdin().read_line(&mut inp).expect("Failed to read line");
//     inp.trim().parse().expect("Please enter a valid number!")
// }
// fn input() -> i32 {
//     loop {
//         let mut inp = String::new();
//         io::stdin().read_line(&mut inp).unwrap();
        
//         match inp.trim().parse() {
//             Ok(num) => return num,
//             Err(_) => println!("সঠিক number দিন!"),
//         }
//     }
// }

fn add(a:i32 , b:i32)->i32
{
    return a+b;
}
fn sub(a:i32, b:i32)->i32
{
    return a-b;
}
fn mul (a:i32, b:i32)->i32
{
    return a*b;
}
fn div (a:i32, b:i32)->i32
{
    return a/b;
}
fn main (){
    let val = input();
    let a = input();
    let b = input ();
    let result=match val{
        1=>add(a,b),
        2=>sub(a,b),
        3=>mul(a,b),
        4=>div(a,b),
        _=>{
        println!("something else");
         0    
        }
    };
    println!("Result = {}", result);
}
