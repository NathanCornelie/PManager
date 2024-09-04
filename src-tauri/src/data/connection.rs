use rusqlite::{Connection,Result};

pub fn establish_connection()->Result<Connection>{
    let conn = Connection::open("pmanager.sqlite")?;
    Ok(conn)
}

