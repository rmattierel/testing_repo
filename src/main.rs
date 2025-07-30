#[macro_use]
extern crate ini;

#[derive(Debug)]
struct Database {
    db_type: String,
    db_host: String,
    db_name: String,
    db_port: String,
    username: String,
    password: String
}

impl Database {
    fn create_dbstring(d: Database) -> String {
      //let mut conn_string = String::from(&d.db_type);
      let mut conn_string = String::from("Ricky");
        // d.db_type,
        conn_string.push_str("://");
        conn_string.push_str(&d.username);
        conn_string.push_str(&d.password);
        conn_string.push_str(&d.db_host);
        conn_string.push_str(&d.db_port);
        conn_string.push_str(&d.db_name);
        
        println!("Checking: {}", conn_string);
        conn_string
    }
}

fn main() {
    // The ini file with all. 
    let map = ini!("file.ini");

    // Retrieve Database parameters.
    let my_database = Database {
      db_type: map["database_info"]["db_type"].clone().unwrap(),
      db_host: map["database_info"]["db_host"].clone().unwrap(),
      db_name: map["database_info"]["db_name"].clone().unwrap(),
      db_port: map["database_info"]["db_port"].clone().unwrap(),
      username: map["login_info"]["username"].clone().unwrap(),
      password: map["login_info"]["password"].clone().unwrap()
    };
    
    println!("This is the value {:?}", Database::create_dbstring(my_database))
}
