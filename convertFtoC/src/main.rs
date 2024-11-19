use std::io;
use std::cmp::Ordering;
// Prmarily intended to convert farenheit to celsius
//    * Input from command line is parsed and converted and response printed
//    * When user feels to exit they can enter `Quit`
fn main() {
    let input_explainer:String = String::from("Enter the temperature in Farenheit:");
    let loop_exiter:String = String::from("Quit");
    'main_loop: loop {
        println!("{}",input_explainer);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("Exit I see :{loop_exiter}");
        match input.cmp(&loop_exiter) {
                Ordering::Equal => break 'main_loop,
                Ordering::Less | Ordering::Greater => println!("number")
        }
        let input = input.trim().parse().expect("Please enter a number");
        println!("Converted {0} to Celsius {1}",input,converter(input));
    }

}

fn converter(faren:f32) -> f32 {
    ( faren - 32f32 ) * ( 5f32 / 9f32 )
}
