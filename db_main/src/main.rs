use db_core::{self, datastore::datastore::KVCommands};

#[tokio::main]
async fn main() {
    
    // let path : &str = "./tmp";
    // let instance = db_core::datastore::rocks_db::Datastore::new(path).await.unwrap();
    // let key = bincode::serialize("key1").unwrap();
    // let value = bincode::serialize("value1").unwrap();
    // match instance.add_kv(key,value) {
    //     Ok(_) => println!("all good"),
    //     Err(e) => println!("{:?}",e),
    // };  
    // let key = bincode::serialize("key2").unwrap();
    // let value = match instance.get_kv(key) {
    //     Ok(v) => v,
    //     Err(e) => {
    //         println!("{:?}",e.to_string());
    //         vec![]
    //     },
    // };

    // let value: String = bincode::deserialize(&value).unwrap();

    // println!("got value: {}",value)

    loop {
        let mut line = String::new();
        let text = std::io::stdin().read_line(&mut line).unwrap();

        // Todo: parse the command given
        
        // Todo: perform the given command.
    }
    
   
}
