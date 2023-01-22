
use std::collections::HashMap;

use super::block::Block;

pub  struct Database{
 
}

impl Database {
    pub fn Connect(){
        

    }

    // this creates a document which is a group of related data stored 
    //in one place(file)
    pub fn CreateDocument(){
        let mut values = HashMap::new();
        values.insert(String::from("name"), String::from("Josh yenu"));
        values.insert(String::from("age"), String::from("23"));
        let mut userDoc :Vec<Block> = vec![];
        userDoc.push(Block{values: values});
        print!("{:?}",userDoc)
    }


    //this saves serialized set of blocks and writes them to a file32032
    pub fn SaveData(){

    }
}
pub fn conn (){

}  





