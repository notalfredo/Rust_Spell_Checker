
use spell_checker::*;
use std::io::{self , Write};
fn main() 
{
    let mut first = String::new();
    let mut second = String::new();



    let mut dictname = String::new();
    let mut testname = String::new();

    let mut print_on : u32;


    while 1 == 1 
    {
        print!("Enter First String : ");
        io::stdout().flush().unwrap();
        first = spell_checker::ask_user_string();


        print!("Enter Second String : ");
        io::stdout().flush().unwrap();
        second = spell_checker::ask_user_string();
        
        //println!("First string = {}  second String = {} " , first , second);

        if first.contains("-1") && second.contains("-1")
        {
            break;
        }
        else
        {
            let distance = edit_distance(&first, &second , 1);
            println!("Edit Distance = {}" , distance);

            println!("===================================================================");

        }

    }

    println!("\nPart 2 - spell check a file.");
    io::stdout().flush().unwrap();

    //print_on = spell_checker::ask_user_number();

    /* 
    print!("Enter the dictionary file name: ");

    print!("\nEnter the dictionary file name: ");
    io::stdout().flush().unwrap();

    dictname = ask_user_string();

    

    print!("Enter the text file name: ");
    io::stdout().flush().unwrap();

    testname = ask_user_string();
    */

    print!("Enter string to be spell - checked: ");
    io::stdout().flush().unwrap();

    testname = ask_user_string();


    //spell_checker::spell_check(&testname, &dictname, 0);
    spell_checker::spell_check(&testname, "dbig.txt", 0);



}
