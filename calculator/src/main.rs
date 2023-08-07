use std::io::stdin;
fn main() {
    loop {
        let mut operation_input = String::new();
        let mut num1_input = String::new();
        let mut num2_input = String::new();

        println!("Your First Number: ");
        stdin().read_line(&mut num1_input).unwrap();
        let num1 = num1_input.trim().parse::<f64>().unwrap();

        println!("Your Second Number: ");
        stdin().read_line(&mut num2_input).unwrap();
        let num2 = num2_input.trim().parse::<f64>().unwrap();

        print!(
            "You can enter the number of operation you would like to do
    1 - Add
    2 - Substract
    3 - Multiply
    4 - Divide

    5- Finish
    your input:
    "
        );
        std::io::stdin().read_line(&mut operation_input).unwrap();

        let operation;

        match operation_input.as_str().trim() {
            "1" => operation = Operation::Add(num1, num2),
            "2" => operation = Operation::Substract(num1, num2),
            "3" => operation = Operation::Multiply(num1, num2),
            "4" => operation = Operation::Divide(num1, num2),
            "5" => {
                break;
            }
            _ => {
                println!("please only choose from list");
                continue;
            }
        }

        let result = calculate(operation);
        println!("--------Your result is: {result}---------- \n");
    }
}

enum Operation {
    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}


fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(num1, num2) => return num1 + num2,
        Operation::Substract(num1, num2) => return num1 - num2,
        Operation::Multiply(num1, num2) => return num1 * num2,
        Operation::Divide(num1, num2) => return num1 / num2,
    }
}

