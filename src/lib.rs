use core::num;
use std::io::{self, stdin, Write};







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
    print!("Please enter String : ");
    io::stdout().flush().unwrap();

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
    let number_of_dashes = ((length +2) * 3) + length+1;
    for i in 0..=number_of_dashes
    {
        print!("-"); io::stdout().flush().unwrap();
    }
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


    println!("first = {} second = {}" , first_len , second_len);


    //let mut vec = vec![vec![0 ; first_len + 1]; second_len + 1];
    let mut vec = vec![vec![0 ; second_len + 1]; first_len + 1];


    for i in 0..=first_len
    {
        vec[i][0] = i;
    }
    
    for j in 0..=second_len
    {
       vec[0][j] = j;
    }




////////////////////////


    for i in 1..=first_len
    {
        for j in 1..=second_len
        {
            //let not_equal : i32;
            //let equal : i32;

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

    


    print_vec(&vec, first_len , second_len  , first , second);


    

    return vec[first_len][second_len];
}


