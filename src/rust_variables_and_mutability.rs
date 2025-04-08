
/*

        Rust Variable Declaration
        We use the let keyword to declare a variable in Rust.
        
        let age = 31;
        
        mut
   
        let mut age = 31;  > this is mutable
        let age = 31;      > this is immutable cannot be changed
   
   
        ===========================================================
        |           Rules for Naming Variables in Rust            |
        ===========================================================
        
        
        1. Rust is a case sensitive language. Hence, lowercase variables and uppercase variables are different. For example,
 
        age is different from AGE
        name is different from Name



        2. Variables must start with either a letter or an underscore. For example,

        let age = 31;     	// valid and good practice
        let _age = 31;    	// valid variable 
        let 1age = 31;    // inavlid variable

        3. Variable names can only contain letters, digits and an underscore character. For example,

        let age1 = 31;        // valid variable
        let age_num = 31;     // valid variable
        let s@lary = 52352;   // invalid variable

        4. Use underscore if we need to use two words as variable names. For example,

        let first name = "Jack";    // invalid variable
        let first_name = "Jack";    // valid variable
        let first-name = "Jack";    // invalid variable


        Rust Constants

        A constant is a special type of variable whose value cannot be changed. We use the const keyword to create constants in Rust.

        const PI: f32 = 3.14;

        Note: As per Rust's naming convention, we use uppercase for the name of constants.

 */
fn main(){

    let name_first ="first name"; // > immutable variable can't update value
    println!("{}", name_first);
    // name_first ="new first name"; > cannot assign twice to immutable variable
    
    let mut name_last ="last name";
    println!("{}", name_last);

    name_last ="new last name";
    println!("{}", name_last);

    const PORT:u32 = 8080;
    println!("Port: {}", PORT);
    
    /*

    output:

    first name
    last name
    new last name
    Port: 8080

    */
}

