use std::collections::HashMap;

struct Todo {
    // use rust built in HashMap to store key-val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    // defining a 'new' function will return a 'Result' that is either a 'Todo stuct' or an
    // 'io::error'
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true) // create the file if not present
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // we pass true as value
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specific an action");
    let item = std::env::args().nth(2).expect("Please specific an item");
    println!("{:?}, {:?}", action, item);

    // instantiate a struct
    let mut todo = Todo::new().expect("Initialisation of db is failed");

    //let mut todo = Todo::new().expect("Initialisation of db failed");

    //let mut todo = Todo {
    //    map: HashMap::new(),
    //};

    if action == "add" {
        // we call the Todo insert method using '.' notation
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurer: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurer: {}", why),
            },
        }
    }
}
