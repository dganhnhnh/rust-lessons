use std::collections::HashMap;
#[derive(Debug)]
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
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades_result=Vec::new();
        for (student, grade) in &self.students{
            grades_result.push(*grade);
        }
        grades_result.sort();
        grades_result.dedup();
        grades_result
    }


    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut same_grades=Vec::new();
        for (student, stu_grade) in &self.students{
            if *stu_grade == grade {
                same_grades.push(student.to_string());
            }
        }
        same_grades.sort();
        same_grades
    }
}

fn main(){

}