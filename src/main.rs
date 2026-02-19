// 1. The main function is always where a Rust program starts.
fn main() {
    // 2. Variables are IMMUTABLE (unchangeable) by default. 
    // We can't change 'system_name' later.
    let system_name = "Marauder"; 

    // 3. To make a variable changeable, we MUST use the 'mut' keyword.
    let mut battery_level = 100; 

    // The 'println!' macro uses {} as placeholders for variables.
    println!("Booting up system: {}", system_name);
    println!("Initial battery: {}%", battery_level);

    // Let's simulate running a scan that drains the battery.
    println!("Running network scan...");
    battery_level -= 15; // This works because we used 'mut' above!

    // 4. Calling another function and passing our variable to it.
    let remaining_time = calculate_uptime(battery_level);

    println!("Scan complete. Battery at {}%.", battery_level);
    println!("Estimated uptime remaining: {} minutes.", remaining_time);
}

// 5. A custom function. We must declare the type of the input (i32 is a standard integer) 
// and the type of the return value (-> i32).
fn calculate_uptime(battery: i32) -> i32 {
    // In Rust, you don't need the 'return' keyword for the last line of a function.
    // Notice there is NO semicolon at the end of this line! 
    // Leaving off the semicolon tells Rust "this is the value to return".
    battery * 3 
}
