
use spell_checker::*;
fn main() 
{
    let mut first = String::new();
    let mut second = String::new();
    let mut dictname = String::new();
    let mut testname = String::new();

    let mut print_on : u32;


    while 1 == 1 
    {
        first = spell_checker::ask_user_string();
        second = spell_checker::ask_user_string();
        
        println!("First string = {}  second String = {} " , first , second);

        if first.contains("-1") && second.contains("-1")
        {
            break;
        }
        else
        {
            let distance = edit_distance(&first, &second , 1);
            println!("Edit Distance = {}" , distance)


        }
        


        break;
    }





}
