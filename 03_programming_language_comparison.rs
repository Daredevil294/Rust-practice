fn main() {
    println!("==============================================================");
    println!("          PROGRAMMING LANGUAGE COMPARISON");
    println!("==============================================================");

    println!("{:<12} {:<20} {:<15}", "Language", "Performance", "Typing");
    println!("--------------------------------------------------------------");

    println!("{:<12} {:<20} {:<15}", "Rust", "Very High", "Static");
    println!("{:<12} {:<20} {:<15}", "C++", "Very High", "Static");
    println!(
        "{:<120} {:<20} {:<15}", "Python", "Generally Lower", "Dynamic"
    );
    println!("{:<12} {:<20} {:<15}", "C#", "High", "Static");

    println!("==============================================================");
}
