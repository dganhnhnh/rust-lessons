use std::collections::HashMap;

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School{
            students: HashMap::new()       
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // unimplemented!()
        
        
    }

    pub fn grades(&self) -> Vec<u32> {
        unimplemented!()

    }


    pub fn grade(&self, grade: u32) -> Vec<String> {
        unimplemented!()

        
    }
}

fn main(){

}