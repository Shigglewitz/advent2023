mod day1;

fn main() {
    println!("Hello, world!");
    
    let total = day1::solve("real_2.txt");
    // 55834
    println!("total is {}", total.to_string());

    println!("Farewell, world!");
}
