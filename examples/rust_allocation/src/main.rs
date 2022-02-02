fn main() {
    let mut line = String::new();
    print!("Enter total number of students:");
    std::io::stdin().read_line(&mut line).unwrap();

    let students_amount: u32 = line.trim().parse().expect("We need an integer here");

    let mut class = Vec::new();

    println!("Enter grades for students");

    for student in 0..students_amount {
        print!("student{} :", student + 1);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        println!();

        let grade: u32 = line.trim().parse().expect("We need an integer here");

        class.push(grade);
    }

    println!("Display grades for students");

    for (index, grade) in class.iter().enumerate() {
        println!("student{} : {}", index + 1, grade);
    }
}
