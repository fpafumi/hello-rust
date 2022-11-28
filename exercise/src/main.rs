fn main() {
    fibonacci_exercise();
    temperature_exercise();
}

//Generate the nth Fibonacci number.
fn fibonacci_exercise() {
    let mut n: i32 = 50;
    let mut first: i64 = 0;
    let mut second: i64 = 1;
    let mut temp: i64;
    while n > 0 {
        temp = first;
        first += second;
        second = temp;
        n -= 1;
    }
    println!("The fibonacci result is {first}");
}

//Convert temperatures between Fahrenheit and Celsius.
fn temperature_exercise() {
    const DEGREE: f32 = 40.0;
    let result_fahreneheit: f32 = temperature_convert(DEGREE, "Fahrenheit");
    let result_celsius: f32 = temperature_convert(DEGREE, "Celsius");
    println!(
        "{DEGREE} degree Celsius equivalent to {} degree Fahrenheit",
        result_fahreneheit
    );
    println!(
        "{DEGREE} degree Fahrenheit equivalent to {} degree Celsius",
        result_celsius
    );
}
fn temperature_convert(temperature: f32, type_to_convert: &str) -> f32 {
    if type_to_convert == "Celsius" {
        return (temperature - 32.0) * 0.555_555_6;
    } else if type_to_convert == "Fahrenheit" {
        return (temperature * 1.8) + 32.0;
    }
    -1.0
}
