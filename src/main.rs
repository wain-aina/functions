fn main() {
    hello_world();
    tell_height(182);
    human_id("Jessie", 22, 1.83);

    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", x);
    let y = add(4,6);
    println!("Value of y is {}", y);
    println!("Your BMI is {:.2}", calculate_bmi(78.01, 1.83))
}

fn hello_world(){
    println!("Hello Rust");
}

fn tell_height(height: u32){
    println!("My height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, I am {}m tall.", name, age, height);
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}