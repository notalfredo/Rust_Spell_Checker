
use spell_checker::*;
use std::io::{self , Write};
fn main() 
{
 
    let mut dictname = String::new();
    let mut testname = String::new();



    

    println!("\nWELCOME TO MY SPELL CHECK PROGRAM");
    println!("Enter any string that you would like to spell check");
    println!("FORMAT RULES : ");
    println!("  - Any letter from A-Z is allowed lowercase and uppercase");
    println!("  - List of symbols to be recognized as separators: space (one white space), comma, dot, exclamation mark, question mark (, .!?).");
    println!("                  ^^^ NO OTHER SEPARATOR CAN BE USED");
    println!("  - ALL sentances must end with a recognized separator");



    print!("Enter string to be spell - checked: ");
    io::stdout().flush().unwrap();

    testname = ask_user_string();


    //spell_checker::spell_check(&testname, &dictname, 0);
    spell_checker::spell_check(&testname, "dbig.txt", 0);



}
