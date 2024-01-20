use std::io;

struct Subject {
    name: String,
    total_marks: f32,
}

fn main() {
    let subjects = &[
        Subject { name: String::from("English"), total_marks: 100.0 },
        Subject { name: String::from("Urdu"), total_marks: 100.0 },
        Subject { name: String::from("Maths"), total_marks: 100.0 },
        Subject { name: String::from("Physics"), total_marks: 85.0 },
        Subject { name: String::from("Computer"), total_marks: 75.0 },
        Subject { name: String::from("Pak-Study"), total_marks: 50.0 },
        Subject { name: String::from("Tarjuma-tul-Quran"), total_marks: 50.0 },
    ];

    let mut total_marks_obtained = 0.0;

    let mut user_subjects: Vec<Subject> = Vec::new();

    for subject in subjects {
        let marks = get_user_input(&format!("Enter marks for {}: ", subject.name));
        total_marks_obtained += marks;

        let user_subject = Subject {
            name: subject.name.clone(),
            total_marks: subject.total_marks,
        };

        user_subjects.push(user_subject);
    }

    let total_percentage = calculate_percentage(total_marks_obtained, subjects.iter().map(|s| s.total_marks).sum());

    println!("\nSubject-wise Percentage:");
    for subject in &user_subjects {
        let subject_percentage = calculate_percentage(subject.total_marks, subject.total_marks);
        println!("{}: {:.2}%", subject.name, subject_percentage);
    }

    println!("\nTotal Percentage: {:.2}%", total_percentage);
}

fn get_user_input(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(marks) if marks >= 0.0 && marks <= 100.0 => return marks,
            _ => println!("Invalid input. Please enter marks between 0 and 100."),
        }
    }
}

fn calculate_percentage(obtained_marks: f32, total_marks: f32) -> f32 {
    (obtained_marks / total_marks) * 100.0
}
