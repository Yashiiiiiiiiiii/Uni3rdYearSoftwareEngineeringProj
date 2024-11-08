use std::io::*;

use mysql::*;
use colored::Colorize;




fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://sql2102675:7377HBPLYpHE@lochnagar.abertay.ac.uk/sql2102675".trim();
    let pool = Pool::new(url)?;

    let conn = pool.get_conn()?;
    // a_id, a_name, a_sysname, a_model, a_type, a_manufactorer, a_ipaddress, a_purchasedate, a_note

    //let mut tx = conn.start_transaction(TxOpts::default())?;
    

    //tx.commit()?;

    program(&conn);




    Ok(())
}



fn program(conn: &PooledConn){

    let run = true;    
    println!("Welcome!");
    println!("Type Help For Info");

    let mut input = String::new();
    while run{
        println!("Command: ");
        let _= stdout();
        stdin().read_line(&mut input).expect("err in main");
        input = input.trim().to_string();
        command(&conn, &input);
        input = String::new();
    }
}



fn command(_conn: &PooledConn,  input: &String) -> bool{

    if input == "help" || input == "Help"{
        println!("Current Commands:");
        println!("help - This one - 'help'");
        println!("exit - Immediately Closes the Program - '{}'", "exit".red());
        println!("add - Used to start the adding process for a given db - '{} {} {}' ", "Add".red(), "Department/Employee/Asset".green(), "''".purple());
        println!("del - Used to start the deletion process for a given db - 'del'");
        println!("view - View a given db - 'view'");
        println!("edit - Edit a given db - 'edit'");
        return true;
    }
    else if input == "exit"{
        //close the terminal and program
        std::process::exit(0);
        
    }
    else if input == "clear"{
        //clear terminal and jump to [0,0] of terminal space
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        
    }

    let cparsed = input.split_whitespace();
    let mut parr: Vec<String> = vec![];
    for part in cparsed{
        parr.push(part.to_string());
    }

    let _len = parr.len();
    //println!("Length of input is {}", len);
    

    return true;
}
