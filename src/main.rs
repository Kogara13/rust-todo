use rusqlite::{ffi::Error, Connection, Result, Statement, params};

mod commands;

#[derive(Debug)]
struct Entry {
    row: u32,
    description: String,
    env: String,
    status: bool,
}

fn main() -> Result<()>{
    let path = "./database.db3";
    //Connect to local sqlite database at "path". If it doesn't exist, the file is created
    let conn = Connection::open(path)?;

    //Check if the table exists
    check_table(&conn)?;

    let task = Entry {
        row: 1,
        description: "Finish your homework".to_string(),
        env: "Room".to_string(),
        status: false,
    };

    conn.execute(commands::INSERT_INTO_TABLE, 
        (&task.row, &task.description, &task.env, &task.status),
    )?;

    //HOW TO PROPERLY RUN LIST()
    match list(&conn) {
        Ok(()) => {println!("Function ran properly");}
        Err(e) => {println!("Error with listing: {}", e);}
    }
    
    Ok(())
}

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
            conn.execute(commands::CREATE_TABLE, params![])?;
            Ok(()) 
        }
        Err(e) => { 
            println!("Error in checking existence {}", e);
            Err(e) 
        }
    }
}


//Function to list existing entries in database
fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM list")?;
    let list_iter = stmt.query_map([], |row| {
        Ok(Entry {
            row: row.get(0)?,
            description: row.get(1)?,
            env: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    for task in list_iter {
        println!("{:?}", task.unwrap());
    }
    Ok(())
}

//Function to add entry to the lst
//fn add() -> Result<()>{

//}