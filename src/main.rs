use errors::CustomError;
use rusqlite::{Connection, Result, Statement, params};
use std::env;
use std::error::Error;

mod query;
mod errors;

#[derive(Debug)]
struct Entry {
    description: String,
    status: bool,
}

fn main() -> Result<()>{
    let path = "./database.db3";
    //Connect to local sqlite database at "path". If it doesn't exist, the file is created
    let conn = Connection::open(path)?;
    //Collect any command line arguments into a vector
    let args: Vec<String> = env::args().collect();
    //Check if the table exists
    check_table(&conn)?;

    //Check the user input
    // Need to convert args from String to &str
    match args[1].as_str() {
        "-a" | "--add" => {
            match add(&conn, &args){
                Ok(()) => {},
                Err(_e) => println!("Something went wrong"),
            }
        
        }, // Add a new entry //CHECK BACK THAT THIS IS RIGHT
        "-r" | "--remove" => {}, // Remove an entry
        "-c" | "--check" => {}, // Check an entry as complete
        "-l" | "--list" => {list(&conn)?;}, // List existing entries //CHECK BACK THAT THIS IS RIGHT
        "-h" | "-help" | _ => {}, // List instructions
    }


    //let task = Entry {
    //    description: "Finish your homework".to_string(),
    //    status: false,
    //};

    //conn.execute(query::INSERT_INTO_TABLE, 
    //    (&task.description, &task.status),
    //)?;

    //HOW TO PROPERLY RUN LIST()
    //match list(&conn) {
    //    Ok(()) => {},
    //    Err(e) => println!("Error with listing: {}", e),
    //}
    
    Ok(())
}

  // ==================================================================================== //
 // ================================Main Functions====================================== //
// ==================================================================================== //

// Function to add entry to the list
fn add(conn: &Connection, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let len = args.len();
    
    if args.len() <= 2 {
        return Err(Box::new(CustomError::new("Not enough arguments")));
    } 

    let mut dick: Vec<String> = Vec::new();
    for i in 2..len {
        // Create a new vector of Strings with the words we want to add as the task
        dick.push(args[i].to_string());
    }
    
    //Use the new vector of strings to create the concatenated task string
    let full_task = dick.join(" ");

    // Setup SQL parameters within a struct
    let params = Entry {
        description: full_task,
        status: false,
    };

    // Execute SQL INSERT statement
    conn.execute(query::INSERT_INTO_TABLE, 
        (&params.description, &params.status),
    )?;

    // Run list function
    match list(&conn) {
        Ok(()) => {},
        Err(e) => println!("Error with listing: {}", e),
    }
    

    Ok(())
    // Nested function to prepare statement with environment entry

}

// Function to remove an existing entry
// (Could be the list_iter statement in list(), but just remove the desired row instead of printing everything)


// Function to check an entry as complete


//Function to list existing entries in database
fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM list")?;
    let list_iter = stmt.query_map([], |row| {
        Ok(Entry {
            description: row.get(1)?,
            status: row.get(2)?,
        })
    })?;

    let mut row = 0;
    for task in list_iter {
        row += 1;
        println!("{}: {:?}", row, task.unwrap());
    }
    Ok(())
}


// Function to print help statement


//Function to create table if it doesn't already exist
fn check_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    //Prepare a statement to list the existing tables within the sqlite database. 
    let mut stmt: Statement = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    //Use match on the results of the statement to see if at least one table exists (the only one that will be created)
    match stmt.exists(params![]) {
        Ok(true) => {
            println!("Table found");
            Ok(()) 
        },
        Ok(false) => { 
            conn.execute(query::CREATE_TABLE, params![])?;
            Ok(()) 
        }
        Err(e) => { 
            println!("Error in checking existence {}", e);
            Err(e) 
        }
    }
}