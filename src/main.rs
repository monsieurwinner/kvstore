use std::collections::HashMap;
fn main() {
   let mut argu = std::env::args().skip(1);
   let key = argu.next().unwrap();
   let value = argu.next().unwrap();

   println!("The key is {} and the value is {}", key,value);
   // Let create a file and save in our project dir using the std:fs library
   //let contents = format!("{}\t{}\n",key,value);
   //std::fs::write("kv.db" , contents);

   let mut database = Database::new().expect("Creating DB failed");
   //database.insert(key, value);
   database.insert(key.to_uppercase(), value.clone()); 
   database.insert(key, value);
   database.flush().unwrap();
}

struct Database{
    map: HashMap<String,String>,
}

// Impl is a way of creating my <Database> class
impl Database {

    //Here we can define our functions with fn
    //The new function is to create a record
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        //We wanna read a file(our database file and output some key and value pair)
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");

            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database{ map : map })
    }

    // Insert record into our KvStore database iwith a method
    // Mehtods are similar to function and declared using fn. But the 1st parameter of a method is always "self"
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key,value); 
    }

    //Flush method
    fn flush(self)-> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
            //let kvpairs = format!("{}\t{}\n", key, value);
            //contents.push_str(&kvpairs);
        }
        std::fs::write("kv.db", contents)
        //todo!()
    }
}

