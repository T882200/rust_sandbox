pub fn run() {
    // print to console
    println!("Hello, world! from the print.rs file");
   
    // basic formatting 
    println!("{} is from {}", "Idan", "Beitar");
    
    // positional arguments
    println!("{0} is from {1}, and {0} likes to {2}", "Idan", "Beitar", "code");
    
    // named arguments
    println!("{name} likes to {activity}", name="John", activity="play");
    
    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 , 10 , 10);
}
