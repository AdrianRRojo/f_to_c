use std::io;


fn main() {
    println!("Hello welcome to my Fahrenheit to Celsius converter in the rust programming language");
    // The math: C = (F - 32) × 5/9
    // F = C × (9/5) + 32

    // define the users input
    let mut temp = String::new();
    println!("Please enter a temperature");
    
    //read the users answer
    io::stdin()
        .read_line(&mut temp)
        .expect("Enter a number!");

    let mut converter = String::new();
    println!("Is the temperature you enter in Fahrenheit or Celsius? Please enter F or C");

    io::stdin()
        .read_line(&mut converter)
        .expect("Please enter F or C");

    // assign it to a new variable, so we can convert the string into an int.
    // trim and parse to convert to int
    //.expect to catch non numbers
    let new_variable: u32 = temp.trim().parse().expect("Please enter a number");
    
    //remove trailing whitespace after the entry. 
    let converter = converter.trim();

    // match converter.as_str() {
    //     "F" => let _answer = new_variable - 32 * 5/9,
    //     "C" => println!("C"),
    //     _ => println!("Please enter F or C"),
    // }
        if converter == "F"{
            // println!("ANS: My F");
            let _answer = (new_variable - 32) * 5/9;
            println!("{_answer}°C");
        } else if converter == "C"{
            // println!("ANS: {converter}");
            let _answer = new_variable * 9/5 + 32;
            println!("{_answer}°F");
        }else{
            println!("Please enter F or C");
        }
    // println!("your temp is {temp}");

    // println!("your temp is {_answer}");
}
