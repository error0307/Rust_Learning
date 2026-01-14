#[derive(Debug)]
enum StudentGrade{
    First,
    Second,
    Third,
}

#[derive(Debug)]
enum Subjects{
    English,
    Math,
    Science,
}

struct StudentData{
    id: u32,
    name: String,
    grade: StudentGrade,
}

struct TeacherData{
    id: u32,
    name: String,
    subject: Subjects 
}

trait DataManager{
    fn print_data(&self);
}

impl StudentData {
    fn new(id: u32, name: &str, grade: StudentGrade) -> StudentData {
        StudentData {
            id,
            name: String::from(name),
            grade,
        }
    }
}

impl DataManager for StudentData {
    fn print_data(&self){
        println!("ID: {}, name: {}, grade: {:?}", self.id, self.name, self.grade );
    }
}


impl TeacherData {
    fn new(id: u32, name: &str, subject: Subjects) -> TeacherData {
        TeacherData {
            id,
            name: String::from(name),
            subject,
        }
    }
}

impl DataManager for TeacherData {
    fn print_data(&self){
        println!("ID: {}, name: {}, Subject: {:?}", self.id, self.name, self.subject );
    }
}

fn main(){
    let student = StudentData::new(1, "Keisuke Ota", StudentGrade::Third);
    let teacher = TeacherData::new(2, "John Smith", Subjects::Math);

    let mut data_vec: Vec<Box<dyn DataManager>> = vec![Box::new(StudentData::new(1, "Keisuke Kondo", StudentGrade::First)), Box::new(TeacherData::new(2, "Chad Smith", Subjects::English))];
    data_vec.push(Box::new(student));
    data_vec.iter().for_each(|data| data.print_data()); // Print all data in the vector

}