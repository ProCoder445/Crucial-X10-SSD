
struct Student<'a> {
    grade: i32,
    student_name: &'a str,
}

impl<'a> Student<'a> {
    pub fn student(name: &'a str, grade: i32) -> Self {
     Student {grade,
     student_name: name}
    }

    pub fn grade_letter(self: &Self) -> Result<String, > {
        if(self.grade >= 95){ return "A+".to_string()}
        else if(self.grade >= 90) { return "A".to_string() }
    }
}