mod database;


use database::act::Act;
use database::database::Database;

fn main() {
    println!("Hello, world!");
   
    Act::do_act();
    Database::CreateDocument();
}
