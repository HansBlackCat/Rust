

trait Person {
    fn name(&self) ->String;
}
// Student is supertrait of Person
trait Student: Person {
    fn university(&self) ->String;
}
trait Programmer {
    fn fav_language(&self) ->String;
}
trait ComputerScienceStudent: Programmer + Student {
    fn git_username(&self) ->String;
}

fn computer_science_student_greeting(student: &dyn ComputerScienceStudent) ->String {
    format!(
        "My name is {}, and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    )
}

fn main() {}