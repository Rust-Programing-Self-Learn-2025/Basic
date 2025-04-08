/**
 * 
 * This is the main() function which acts as an entry point of every Rust program. It is always the first code that runs in every Rust program.
 * The body of the function is wrapped inside curly brackets, {}. We will learn more about functions in later chapters.
 * 
 * Here, print! is a macro that prints the text inside double quotes. To learn more about macros, visit Rust Macro.
 *
 * 
 * In Rust, there are two variations of the print:
 *
 *      1. print!()
 *      2. println!()
 * 
 */
fn main(){
   println!("=========== Difference between print and println ================\n");
   println!("===========  print ================ \n");
   print();

   println!("\n\n===========  println ================ \n");
   println();

   println!("\n\n===========  Print Variables ================ \n");
   print_variables();
}

fn print_variables(){
    let age = 31;
    let name = "Jack";
    println!("Age: {}, name: {}", age, name);
    // output: Age: 31, name: Jack
    println!("Age: {1}, name: {0}", age, name);
    // output: Age: Jack, name: 31
    println!("Age: {age}, name: {name}");
    // output: Age: 31, name: Jack

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
