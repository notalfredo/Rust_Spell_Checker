use core::num;
use std::fs;
use std::io::{self, stdin, Write};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn all_possible(cmp_string : &str , our_vec : &Vec<String>  ) -> Vec<String>
{

    let mut possible_fixs: Vec<String> = Vec::new();

    let mut min_dist : usize = 0;
    let mut cur_edit_dist : usize ;

    let mut count = 0;

    for line in our_vec
    {   
        cur_edit_dist = edit_distance(&cmp_string , &line , 0);


        if count == 0
        {
            min_dist = cur_edit_dist;
            possible_fixs.push(line.to_string());
        }

         else if  min_dist == cur_edit_dist
         {
            possible_fixs.push(line.to_string());
         }
         else if min_dist < cur_edit_dist
         {
            min_dist = cur_edit_dist;
            possible_fixs.clear();
            possible_fixs.push(line.to_string());
         }

        count += 1;
    }

    return possible_fixs;
}


fn read_file_line_by_line(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()

    
}

fn print_vector(my_vector : &Vec<String>) 
{
    for i in my_vector {
        println!("{}", i);
    }
}

fn convert_string_to_integer(number : &str) -> i32
{
    let number: i32 = match number.trim().parse()
    {
        Ok(num) => num,
        Err(_) => 
        {
            panic!("Could not read number");
        }
    };

    return number;
}

fn save_first_line_into_string(file_name : &str) -> String
{
    let contents = fs::read_to_string(file_name)
        .expect("could not read file");
    
    return contents;
}







pub fn ask_user_number
() -> u32
{

    print!("Please enter number : ");
    io::stdout().flush().unwrap();

    loop
    {
        let mut user_inpt = String::new();

        io::stdin()
            .read_line(&mut user_inpt)
            .expect("Error ");

        
        let user_inpt: u32 = match user_inpt.trim().parse()
        {
            Ok(num) => num,
            Err(_) => 
            {
                print!("Please enter valid number");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        return user_inpt;
    }
}



pub fn ask_user_string
() -> String
{


    let mut input_string : String = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("could not read input");

    input_string.pop();

    return input_string;

}


pub fn spacer(n : usize) -> &'static str
{
    if n < 10
    {
        return "  ";
    }
    else if n >= 10 && n<= 100
    {
        return " ";
    }
    else
    {
        return "";
    }
}




fn print_dash(length : usize)
{
    /* 
    let number_of_dashes = ((length +2) * 3) + length+1;
    for i in 0..=number_of_dashes
    {
        print!("-"); io::stdout().flush().unwrap();
    }
    */

    let dashes = ((length + 2) * 3) + length + 2;
    print!("{}", "-".repeat(dashes));
    io::stdout().flush().unwrap();
}


pub fn min_distance(left : usize , top : usize , Diagnal : usize) -> usize
{
	//find the smallest

	if left<top
	{
		if Diagnal<left 
		{
			return Diagnal;
		} 
		else 
		{
			return left;
		}
	} 
	else 
	{
		if top<Diagnal
		{
			return top;
		}
		else 
		{
			return Diagnal;
		}
	}
		



}

pub fn print_vec(vec: &Vec<Vec<usize>> , rows : usize , columns : usize , first : &str  , second : &str) 
{


    let first_string = first.to_string();
    let second_string = second.to_string();


    //println!("first = {} second = {}" , rows , columns);

    

    for i in 0..rows+2
    {
        for j in 0..columns+2
        {
            if (i == 0 && j ==0) || (i ==0 && j==1) || (i == 1 && j == 0)
            {
                print!("   |"); io::stdout().flush().unwrap();
                
            }
            else if i ==0 && j > 1
            {
                print!("  {}|", second_string.chars().nth(j-2).unwrap());
                io::stdout().flush().unwrap();
            }
            else if i > 1 && j == 0
            {
                print!("  {}|", first_string.chars().nth(i-2).unwrap());
                io::stdout().flush().unwrap();        
            }
            else if i >= 1 && j >= 1 
            {
                print!("{}{}|" ,spacer(vec[i-1][j-1]) , vec[i-1][j-1]);
                io::stdout().flush().unwrap();          
            }


        }

        println!("");
        print_dash(columns);
        println!("");

    }

}


pub fn edit_distance
(first : &str , second : &str , print_on : u32) 
-> usize
{

    let first_len : usize = first.len();
    let second_len : usize = second.len();


    //println!("first = {} second = {}" , first_len , second_len);
    let mut vec = vec![vec![0 ; second_len + 1]; first_len + 1];

    for i in 0..=first_len
    {
        vec[i][0] = i;
    }
    
    for j in 0..=second_len
    {
       vec[0][j] = j;
    }


    for i in 1..=first_len
    {
        for j in 1..=second_len
        {
            if first.chars().nth(i-1).unwrap() != second.chars().nth(j-1).unwrap()
            {
                let not_equal = min_distance( vec[i-1][j] +1 , vec[i][j-1] +1  , vec[i-1][j-1]+1);
                vec[i][j] = not_equal;
            }
            else if first.chars().nth(i-1).unwrap() == second.chars().nth(j-1).unwrap()
            {
                let equal = min_distance( vec[i-1][j] +1 , vec[i][j-1] +1  , vec[i-1][j-1]+0);
                vec[i][j] = equal;
            }
        }
    }

    if print_on == 1
    {
        print_vec(&vec, first_len , second_len  , first , second);
    }

    return vec[first_len][second_len];
}




pub fn spell_check( testname : &str  , dictname : &str , printOn : u32)
{

    let mut vec = read_file_line_by_line(dictname);
    vec.sort();
    vec.drain(0..1);



    let my_testname = save_first_line_into_string(testname);


    //println!("My first string is ==== |{}|" , my_testname);

    if printOn == 1
    {
        print_vector(&vec);
    }

    let mut temp_store : String = String::new();

    let mut edit_mode : i32 = -1;
    

    for c in my_testname.chars() 
    { 
        
        if ((c == '.') || (c == ',') || (c == '?') || (c == ' ')) && edit_mode == -1
        {
            edit_mode = 0 ;
        }
        else if ((c == '.') || (c == ',') || (c == '?') || (c == ' ') ) && edit_mode == 0
        {
            edit_mode = 1 ;
        }
        else 
        {
            edit_mode = -1 ;
            temp_store.push(c);
        }

        
        if (edit_mode == 0) && (!vec.contains(&temp_store.to_lowercase()))
        {
            let found = all_possible(&temp_store , &vec);
            //println!("Found = {}" , temp_store)
            for line in found{
                println!("{}" , line);
            }


            println!("\n\n");
        }


        //vec.contains(&temp_store);











        if edit_mode == 0
        {
            //println!("edit mode = {}" , edit_mode);
            //println!("{}" , temp_store);
            //if vec.contains(&temp_store.to_lowercase())
            //{
            //    println!("Found = {}" , temp_store);
            //}
            temp_store.clear();
        }
    }
    //temp_store.clear();
    //print!("{}" , temp_store);
    //io::stdout().flush().unwrap();

}