fn main() {
    
    /*-----------------------------------------------------
    -       Compound Data Types
                -can store more than one simple value
                - more than one digit or characters
                - this lesson focuses on string
                - most languages have a string data type
    -------------------------------------------------------*/

    /*-----------------------------------------------------------
    /           Strings
    /               -String slices (&str) - Fixed length strings
    /------------------------------------------------------------*/

    let some_string = "Fixed Length String";
    println!("The text inside the string is \"{}\"", some_string);

    
    /*-----------------------------------------------------------
    /           Strings
    /               - Variable length strings
    /               - adding characters using push()
    /               - removing characters using pop()
    /               - operations on strings like: 
    /                       len()
    /                       capacity()
    /                       contains()
    /                       trim()
    /                       push_str()
    /                       to_string()
    /               - Formatting and concatenating strings
    /------------------------------------------------------------*/

    //variable length string with the "mut" keyword
    let mut growable_string = String::from("This string will grow");    //needed mut keyword to allow it to change
    println!("The text inside the string is \"{}\"", growable_string);

    //adding characters using push()
    growable_string.push('s'); //has to be a char
    //growable_string.push('addmultiplecharacters');        //this does not work
    println!("The text inside the string changed to \"{}\"", growable_string);

    //removing characters using pop()
    growable_string.pop();
    println!("The text inside the string changed to \"{}\"", growable_string);

    // Operations on strings
    growable_string.push_str(" which I will use");
    println!("The text inside the string is \"{} \"", growable_string);

    println!("I am going to tell you some basic things about the strings, 
    Is the string empty? {},
    The length of the string is {},
    The sting has {} bytes,
    Does the string contain the word 'use'? {}", growable_string.is_empty(),
    growable_string.len(),
    growable_string.capacity(),
    growable_string.contains("use"));


    growable_string.push_str("    ");
    println!("The length of the string before the trim is {},
    The length of the string after the trim is {}", growable_string.len(), growable_string.trim().len());
    
    let number = 6;
    println!("The value of the number in string is {}", number.to_string());

    println!("Is the number really a string? {}", number.to_string() == "6");

    let some_char = 'a';
    println!("The character in string is {}", some_char.to_string());

    println!("Is the char really a string? {}", some_char.to_string() == "a");


    // Formatting and Concateneating strings
    let my_name = "Chandler Bartholomew".to_string();
    println!("The string contains my name {}", my_name);
    

}

