use std::{collections::HashMap, ptr::hash};
fn main() {
   let mut argu = std::env::args().skip(1);
   let key = argu.next().unwrap();
   let value = argu.next().unwrap();

   println!("The key is {} and the value is {}", key,value);
   // Let create a file and save in our project dir using the std:fs library
   let contents = format!("{}\t{}\n",key,value);
   std::fs::write("kv.db" , contents);

   let databade = Database::new().expect("Creating DB failed");
}

struct Database{
    map: HashMap<String,String>,
}

// Impl is a way of creating my <Database> class
impl Database {

    //Here we can define our functions with fn
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        //We wanna read a file(our database file and output some key and value pair)
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");

            map.insert(key, value);
        }
        Ok(Database{
            map : HashMap::new(),
        })
    }
}

