use std::io::*;

use colored::Colorize;
use mysql::prelude::*;
use mysql::*;

//consider using enums and structs to represent cases
#[allow(dead_code)]
struct Asset {
    id: i32,
    name: String,
    sysname: String,
    model: String,
    r#type: String,
    manufacturer: String,
    ip: String,
    purchasedate: String,
    note: String,
    employee: String,
}

#[allow(dead_code)]
struct Department {
    id: i32,
    name: String,
    employee: String,
}

#[allow(dead_code)]
struct Employee {
    id: i32,
    firstname: String,
    secondname: String,
    email: String,
    department: String,
    assets: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://sql2102675:7377HBPLYpHE@lochnagar.abertay.ac.uk/sql2102675".trim();
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    // a_id, a_name, a_sysname, a_model, a_type, a_manufactorer, a_ipaddress, a_purchasedate, a_note
    /*
    let mut tx = conn.start_transaction(TxOpts::default())?;
    tx.query_drop(
        "CREATE TABLE Employee(
        id INTEGER PRIMARY KEY,
        firstname TEXT NOT NULL,
        secondname TEXT NOT NULL,
        email TEXT NOT NULL,
        department TEXT NOT NULL,
        assets TEXT
        )",
    )?;

    tx.commit()?;
    */
    program(&mut conn);

    Ok(())
}

fn program(conn: &mut PooledConn) {
    let run = true;
    println!("Welcome!");
    println!("Type Help For Info");

    let mut input = String::new();
    while run {
        println!("Command: ");
        let _ = stdout();
        stdin().read_line(&mut input).expect("err in main");
        input = input.trim().to_string();
        command(conn, &input);
        input = String::new();
    }
}

fn command(conn: &mut PooledConn, input: &String) -> bool {
    if input == "help" || input == "Help" {
        println!("Current Commands:");

        println!("help - This one - '{}'", "help".red());

        println!("exit - Immediately Closes the Program - '{}'", "exit".red());

        println!(
            "\nadd - Used to add to a db - '{} {} {} {}' \n{}\n{}\n{}",
            "[Add]".red(),
            "[Department/Employee/Asset]".green(),
            "[Fields in format e.g. (name,employee)]".purple(),
            "[With values in format e.g. (Some_Name,Some_id)]".yellow(),
            "[NO SPACES IN FIELDS (name,employee) NOT ( name, employee )] && [no need to include id as it will be calculated automatically]".cyan(),
            "[to add more than one in certain fields separate id's by '-' e.g. (Some_Name,1-3-7-37)]".blue(),
            "[Use '_' as a replacement for spaces]".magenta()
        );

        println!(
            "\ndel - Used to delete field(s) from a table- '{} {} {}'",
            "NOT IMPLEMENTED [del]".red(),
            "[Department/Employee/Asset]".green(),
            "[WHERE CONDITION]".purple()
        );

        println!(
            "\nview - View a given db - '{} {} optional: {}'",
            "NOT IMPLEMENTED [view]".red(),
            "[Department/Employee/Asset]".green(),
            "[WHERE CONDITION]".purple()
        );

        println!("edit - COMING SOON - 'edit'");

        return true;
    } else if input == "exit" {
        //close the terminal and program
        std::process::exit(0);
    } else if input == "clear" {
        //clear terminal and jump to [0,0] of terminal space
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
    }

    let cparsed = input.split_whitespace();
    let mut parr: Vec<String> = vec![];
    for part in cparsed {
        parr.push(part.to_string());
    }

    let len = parr.len();
    //println!("Length of input is {}", len);
    let argcon = [String::new(), String::new(), String::new(), String::new()];
    //given some len of input command is limited to that
    if len == 4 {
        //TODO model through struct or enum
        //only possible to be add
        add_query(conn, argcon, parr);
    }

    return true;
}

fn get_id_count(conn: &mut PooledConn, query: String) -> i32 {
    let mut tx = conn
        .start_transaction(TxOpts::default())
        .expect("transaction start failed in get id count");
    let depart: Row;

    if query == "department" {
        depart = tx
            .query_first("SELECT * FROM Department ORDER BY id DESC LIMIT 0, 1")
            .unwrap()
            .expect("fsfw");
        let output = format!("{:?}", depart);
        let ss = output.split(",");
        let mut ssarr: Vec<String> = vec![];
        for part in ss {
            ssarr.push(part.to_string());
            //println!("Args split: {}", part.to_string());
        }
        //we only care about the first output

        let a = replace_nonnumbers(&mut ssarr[0]);
        //println!("{}", a);
        let ret = a.parse::<i32>().expect("Fuck SAKES NO WAY");
        //println!("{}",ret);

        return ret;
    } else if query == "asset" {
        depart = tx
            .query_first("SELECT * FROM ASSET ORDER BY id DESC LIMIT 0, 1")
            .unwrap()
            .expect("Failure in asset row grab");

        let output = format!("{:?}", depart);
        let ss = output.split(",");
        let mut ssarr: Vec<String> = vec![];
        for part in ss {
            ssarr.push(part.to_string());
        }
        //we only care about the first output

        let a = replace_nonnumbers(&mut ssarr[0]);
        //println!("{}", a);
        let ret = a.parse::<i32>().expect("failure to parse into integer");
        //println!("{}",ret);

        return ret;
    } else if query == "employee" {
        depart = tx
            .query_first("SELECT * FROM Employee ORDER BY id DESC LIMIT 0, 1")
            .unwrap()
            .expect("fsfw");
        let output = format!("{:?}", depart);
        let ss = output.split(",");
        let mut ssarr: Vec<String> = vec![];
        for part in ss {
            ssarr.push(part.to_string());
        }
        //we only care about the first output

        let a = replace_nonnumbers(&mut ssarr[0]);
        //println!("{}", a);
        let ret = a.parse::<i32>().expect("Fuck SAKES NO WAY");
        return ret;
    } else {
        return -1;
    }
}

fn send_query(conn: &mut PooledConn, query: String) {
    //was told to not include things not mentioned in the brief so i will not prevent sql injection or otherwise

    let mut tx = conn
        .start_transaction(TxOpts::default())
        .expect("transaction start failed in send query");
    tx.query_drop(query).expect("Error on query send");
    tx.commit().expect("Error on commit : Send Query");
}

#[allow(dead_code)]
fn send_query_with_return(conn: &mut PooledConn, query: String) -> String {
    //was told to not include things not mentioned in the brief so i will not prevent sql injection or otherwise
    let mut tx = conn
        .start_transaction(TxOpts::default())
        .expect("transaction start failed in send query");
    tx.query_drop(query).expect("Error on query send");
    tx.commit().expect("Error on commit : Send Query");

    "Not implemented".to_string()
}

fn replace_nonnumbers(input: &mut String) -> String {
    //let s = input.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','a','b','c','d','d','d','d','d','d','d','d','d',][..], ""); OLD
    let s = input;
    s.retain(|c| {
        !r#"()," .;:abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'{}[]"#.contains(c)
    });
    s.to_string()
}

fn add_query(conn: &mut PooledConn, mut argcon: [String; 4], parr: Vec<String>) {
    //prove first input is add
    
    if parr[0] == "add" || parr[0] == "Add" || parr[0] == "ADD" {
        //is add so continue
        argcon[0] = "INSERT INTO".to_string();
        //second input is department, employee or asset
        if parr[1] == "department" {
            argcon[1] = "Department".to_string();
            //max 2 inputs excluding id

            //get current highest added
            let mut high = get_id_count(conn, "department".to_string());
            //println!("{}",high);

            //we're adding a department so -> (id,name,employee), employee does not need to be filled immediately so name is all thats necessary

            // i32 "high" is equal to current highest id so high++
            high += 1;
            // now we use the given values to insert
            let s3 = &parr[2];
            if s3.contains(",") {
                //if theres a comma then we have 2 inputs otherwise only 1
                //check we have the correct input
                //split args and clean up

                //only use for this is error messages
                let s3split = s3.split(",");
                let mut s3arr: Vec<String> = vec![];
                for part in s3split {
                    s3arr.push(part.to_string());
                    //println!("Args split: {}", part.to_string());
                }

                let ss1 = s3arr[0].replace("(", "");
                let ss2 = s3arr[1].replace(")", "");

                if ss1.contains("name") {
                    //we have a required argument
                    //check second arg
                    if ss2.contains("employee") {
                        //we have the required arguments
                        //hard coded as theres no need to parse everything
                        argcon[2] = "(id,name,employee)".to_string();

                        //now get the inputs
                        let s4 = &parr[3];
                        let s4split = s4.split(",");
                        let mut s4arr: Vec<String> = vec![];
                        for part in s4split {
                            s4arr.push(part.to_string());
                            //println!("Args split: {}", part.to_string());
                        }
                        let sss1 = s4arr[0].replace("(", "");
                        let sss2 = s4arr[1].replace(")", "");

                        //sss1 is name and sss2 is employees so
                        argcon[3] = "VALUES (".to_owned()
                            + high.to_string().as_str()
                            + ",\""
                            + sss1.to_string().as_str()
                            + "\",\""
                            + sss2.to_string().as_str()
                            + "\")";
                        //println!("Output of argcon is {} {} {} {}", argcon[0], argcon[1],argcon[2], argcon[3]);
                    } else {
                        //args arent correct return help msg
                        println!("expected 'employee' found: {}", ss2.red());
                    }
                } else {
                    println!("expected 'name' found: {}", ss1.red());
                }
            } else {
                //check we have the correct input
                if s3.contains("name") {
                    //we have the required
                    argcon[2] = "(id,name)".to_string();

                    let s4 = &parr[3];
                    let ss4 = s4.replace("(", "");
                    let ss5 = ss4.replace(")", "");
                    //println!("output {}", ss5);

                    argcon[3] = "VALUES (".to_owned()
                        + high.to_string().as_str()
                        + ",\""
                        + ss5.as_str()
                        + "\")";
                    //println!("Output of argcon is {} {} {} {}", argcon[0], argcon[1],argcon[2], argcon[3]);
                    let n = "".to_owned()
                        + argcon[0].as_str()
                        + " "
                        + argcon[1].as_str()
                        + " "
                        + argcon[2].as_str()
                        + " "
                        + argcon[3].as_str();
                    send_query(conn, n);
                }
            }
        } else if parr[1] == "employee" {
            argcon[1] = "Employee".to_string();
            //max 6 inputs
        } else if parr[1] == "asset" {
            //max 9 inputs, i cannot be bothered working out that nested if statement list so
            argcon[1] = "ASSET".to_string();
            let options = [
                "name",
                "sysname",
                "model",
                "type",
                "manufacturer",
                "ip",
                "purchase_date",
                "note",
                "employee",
            ];
            let mut given: Vec<String> = vec![];
            let mut regiven: Vec<String> = vec![];
            for o in options {
                if parr[2].contains(o) {
                    //println!("contains: {}",o);
                    given.push(o.to_string());
                    regiven.push(o.to_string());
                }
            }

            let g_len = given.len();
            let mut high = get_id_count(conn, "asset".to_string());
            high += 1;
            argcon[2] = "(".to_owned() + "id" + ",";
            for g in given {
                let s = g + ",";
                argcon[2] += &s;
            }
            //println!("argcon[2]: {}", argcon[2]);
            argcon[2].pop();
            //println!("updated argcon[2]: {}", argcon[2]);
            argcon[2] += ")";

            argcon[3] = "VALUES (".to_owned() + high.to_string().as_str() + ",";
            let ss = parr[3].split(",");
            let mut ar3: Vec<String> = vec![];
            for o in ss {
                ar3.push(o.to_string());
                //println!("{}", o);
            }
            let ar_len = ar3.len();
            if g_len != ar_len {
                //not correct inputs so return to commands section
                return;
            }

            //remove brackets on first and last input
            ar3[0].remove(0);
            ar3[ar_len - 1].pop();

            let mut outputs: Vec<String> = vec![];

            let mut co = 0;
            for check in regiven {
                // fix this its duped code
                if check == "name" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "sysname" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "model" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "type" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "manufacturer" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "ip" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "purchase_date" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "note" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else if check == "employee" {
                    let ch = ar3[co].replace("_", " ");
                    let ie = "\"".to_owned() + ch.as_str() + "\"";
                    outputs.push(ie);
                } else {
                    println!("Some error input: {}", check);
                }
                co += 1;
            }

            for q in outputs {
                let s = q + ",";
                argcon[3] += &s;
            }
            //remove final comma
            argcon[3].pop();

            //finalise args
            argcon[3] += ")";
            //println!("argcon[3]: {}", argcon[3]);
            
            //create query
            let n = "".to_owned()
                        + argcon[0].as_str()
                        + " "
                        + argcon[1].as_str()
                        + " "
                        + argcon[2].as_str()
                        + " "
                        + argcon[3].as_str();

                        println!("argcon: {}", n);

            send_query(conn, n);

        } else {
            //Aborts if no input is met
            println!("Incorrect args");
        }
    }
}
