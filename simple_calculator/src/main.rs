fn main() {
    let add = Operation::Add(5.0,6.0);
    calculate(add);

    let subtract = Operation::Subtract(5.0,6.0);
    calculate(subtract);

    let mul = Operation::Multiply(5.0, 6.0);
    calculate(mul);

    let divide = Operation::Divide(5.0,6.0);
    calculate(divide);
}


enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn calculate(operation: Operation) {
    match operation {
        Operation::Add(a,b) => {
            println!("Result of additon: {}", a + b);
        }
        Operation::Subtract(a,b ) => {
            println!("Result of subtraction: {}", a - b);
        }
        Operation::Multiply(a,b ) => {
            println!("Result of multiplication: {}", a * b);
        }
        Operation::Divide(a,b ) => {
            if b != 0.0 {
                println!("Result of division: {}", a / b);
            } else {
                print!("Division by zero not possible")
            }
        }
    }
}