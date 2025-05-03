fn main() {
    let student_test_grades: [i32; 5] = [100, 88, 77, 66, 99];
    let denominator = student_test_grades.len();
    let numerator: i32 = student_test_grades.iter().sum();
    let result = numerator as i32 / denominator as i32;
    println!("Average score for the class is...\n");
    println!("{}", result);
}
