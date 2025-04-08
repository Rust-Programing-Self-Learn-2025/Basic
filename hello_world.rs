/**
 * 
 * This is the main() function which acts as an entry point of every Rust program. It is always the first code that runs in every Rust program.
 * The body of the function is wrapped inside curly brackets, {}. We will learn more about functions in later chapters.
 * 
 */
fn main(){
    println!("=========== Difference between print and println ================\n");
    println!("===========  print ================ \n");
   print();
   println!("\n\n===========  println ================ \n");
   println();
}

fn print(){
    print!("Hello, World!");
    print!("Hello, World!");
    print!("Hello, World!");
    /*
    * Output:
    * 
    * Hello, World!Hello, World!Hello, World!
    */
}

fn println(){
    println!("Hello, World!");
    println!("Hello, World!");
    println!("Hello, World!");
    /*
    * Output:
    * 
    * Hello, World!
    * Hello, World!
    * Hello, World!
    */
}


/*
 * full Output:
 * 
 * =========== Difference between print and println ================
 * 
 * ===========  print ================ 
 * 
 * Hello, World!Hello, World!Hello, World!
 * 
 * ===========  println ================ 
 * 
 * Hello, World!
 * Hello, World!
 * Hello, World!
 * 
 */
