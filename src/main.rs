use anyhow::Result;
use futures::executor::block_on;

use std::env;
use std::fs;

use parser::types::{ParserError, ParserErrorType, Ticket};

use parser::db::{init_db, insert_into_db};

async fn async_main() -> Result<(), ParserError> {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("An argument must be supplied");
    }

    if args.len() == 2 {
        if &args[1] == "--initial" {
            let _ = init_db();
            return Ok(());
        }
        println!(
            "{}",
            "You must provide the correct nummber of arguments\n".to_string()
                + help_message().as_str()
        );
    }

    if args.len() != 3 || args[1] != "-f" {
        println!(
            "{}",
            "You must provide the correct nummber of arguments\n".to_string()
                + help_message().as_str()
        );
        return Ok(());
    }

    //let file = fs::read_to_string("./src/facturas.in")
    let file = fs::read_to_string(&args[2])
        .map_err(|_| ParserError::new(ParserErrorType::InvalidPath, "", ""))?;

    let ticket = file.trim().parse::<Ticket>().unwrap();
    insert_into_db(&ticket).await?;
    dbg!(ticket);
    Ok(())
}
fn main() {
    let _ = block_on(async_main());
}

fn help_message() -> String {
    r#" 
The options are as follows : 
        -f <path>       the path for a file of tickets to parse and load

        --initial       restart the entire database
                        drop database, and cretate it again, used this
                        option when you use the tool for first time.
    "#
    .to_string()
}
