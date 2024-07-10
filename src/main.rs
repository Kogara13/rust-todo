use rusqlite::{ffi::Error, Connection, Result, Statement, params};
use sdt::env;

mod query;

#[derive(Debug)]
struct Entry {
    row: u32,
    description: String,
    status: bool,
}

fn main() -> Result<()>{
    let path = "./database.db3";
    //Connect to local sqlite database at "path". If it doesn't exist, the file is created
    let conn = Connection::open(path)?;
    //Collect any command line arguments into a vector
    let args: Vec<String> = env::args().collect;
    //Check if the table exists
    check_table(&conn)?;

    //Check the user input
    match &args[1] {
        "-a" | "--add" => {add(&conn, &args)?;}, // Add a new entry //CHECK BACK THAT THIS IS RIGHT
        "-r" | "--remove" => {}, // Remove an entry
        "-c" | "--check" => {}, // Check an entry as complete
        "-l" | "--list" => {list(&conn)?;}, // List existing entries //CHECK BACK THAT THIS IS RIGHT
        "-h" | "-help" | _ => {}, // List instructions
    }


    //let task = Entry {
    //    row: 1,
    //    description: "Finish your homework".to_string(),
    //    env: "Room".to_string(),
    //    status: false,
    //};

    //conn.execute(query::INSERT_INTO_TABLE, 
    //    (&task.row, &task.description, &task.env, &task.status),
    //)?;

    //HOW TO PROPERLY RUN LIST()
    match list(&conn) {
        Ok(()) => {println!("Function ran properly");}
        Err(e) => {println!("Error with listing: {}", e);}
    }
    
    Ok(())
}

  // ==================================================================================== //
 // ================================Main Functions====================================== //
// ==================================================================================== //

// Function to add entry to the list
fn add(conn: &Connection, args: &Vec<String>) -> Result<()> {
    let len = args.len();
    let mut dick: Vec<String> = Vec::new();
    for i in 2..len {
        // Create a new vector of Strings with the words we want to add as the task
        dick.push(args[i].to_string());
    }
    
    //Use the new vector of strings to create the concatenated task string
    let full_task = dick.join(" ");

    

    Ok(())
    // Nested function to prepare statement with environment entry

}

// Function to remove an existing entry


// Function to check an entry as complete


//Function to list existing entries in database
fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM list")?;
    let list_iter = stmt.query_map([], |row| {
        Ok(Entry {
            row: row.get(0)?,
            description: row.get(1)?,
            status: row.get(2)?,
        })
    })?;

    for task in list_iter {
        println!("{:?}", task.unwrap());
    }
    Ok(())
}


// Function to print help statement


//Function to create table if it doesn't already exist
fn check_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    //Prepare a statement to list the existing tables within the sqlite database. 
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
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