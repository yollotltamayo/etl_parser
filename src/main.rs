use futures::executor::block_on;
use colored::Colorize;

use std::env;
use std::fs;

use parser::types::{ParserError, ParserErrorType, Ticket};
use parser::db::{init_db, insert_into_db};
use parser::validator::{validate_ticket};


async fn async_main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("An argument must be supplied");
    }

    if args.len() == 2 {
        if &args[1] == "--initial" {
            let _ = init_db();
            return;
        }
        println!(
            "{}",
            "You must provide the correct number of arguments\n".to_string()
                + help_message().as_str()
        );
    }

    if args.len() != 3 || args[1] != "-f" {
        println!(
            "{}",
            "You must provide the correct nummber of arguments\n".to_string()
                + help_message().as_str()
        );
        return;
    }

    //let file = fs::read_to_string("./src/facturas.in")

    println!("{}","Loading File (ETL process)".cyan());
    let file = match fs::read_to_string(&args[2])
        .map_err(|_| ParserError::new(ParserErrorType::InvalidPath, "", &args[2])) {
            Ok(value) => value, 
            Err(e) => {
                println!("{}","File loading don't succeed  ❌".cyan());
                println!("{}","Found the following error(s) ".red());
                dbg!(e);
                return;
            }
        };
    println!("{}","File loading succeed ✅".cyan());

    println!("{}","Parsing File".purple());

    let ticket = match file.trim().parse::<Ticket>() {
        Ok(ticket) => {
            let ticket_has_error = ticket
                .facturas
                .iter()
                .any(|factura| factura.is_err());
            if ticket_has_error  {
                println!("{}","Parsing of the file succeed  ✅ with some errors ❌".purple());
            }else{
                println!("{}","Parsing of the file succeed  ✅".purple());
            }
            println!("{}","Showing the parsed result :".purple());
            dbg!(ticket)
        },
        Err(e) =>{
            println!("{}","Parsing of the file not succeed  ❌".red());
            println!("{}","Found the following error(s) ".red());
            dbg!(e);
            return;
        }
    };
   
    println!("{}","Validating File".blue());
    let ticket = match validate_ticket(ticket) {
        Ok(ticket) => {
            println!("{}","Validation of the file succeed  ✅".blue());
            ticket
        },
        Err(errors) => {
            println!("{}","Validation of the file don't succeed  ❌".blue());
            println!("{}","Found the following error(s):".red());
            dbg!(errors) ;
            return;
        }
    };

    println!("{}","Inserting into database ✅".cyan());
    match insert_into_db(&ticket).await {
        Ok(_) => {
            println!("{}","Insertion of ticket succeed ✅".cyan());
        },
        Err(e) => {
            println!("{}","Insertion of ticket don't succeed  ❌".cyan());
            println!("{}","Found the following error(s):".red());
            dbg!(e) ;
        }
    }
}
fn main() {
    block_on(async_main());
}

fn help_message() -> String {
    r#" 
The options are as follow : 
        -f <path>       The path for a file of tickets to parse and load
                        to database

        --initial       Restart the entire database
                        drop database, and cretate it again, used this
                        option when you use the tool for first time.
    "#
    .to_string()
}
