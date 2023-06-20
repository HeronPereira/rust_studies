use std::io;

fn convert_to_int(data_input: & String) -> i32
{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
  /*  let x: i64 = 26; //numero inteiro signed
    let f: f32 = 3.4; // variavel float 32 bits
    let b: bool = true; // variavel boolean
    let mut name = "Heron";
    name = "Heron Pereira";
    println!("Hello, {}!", name);*/
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Error reading num1");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Error reading num2");
    
    if convert_to_int(&mut num1) > convert_to_int(& mut num2)
    {
        println!("num1 is bigger!");
    }
    else
    {
        println!("num1 is not bigger than num2");
    }

}
