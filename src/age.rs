use std::io;
use chrono::Utc;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
struct Age {
    years: i32,
    months: i32,
    days: i32,
    hours: i32,
    seconds: i32,
}

fn main() {
    let birth_year = get_user_input("Enter your birth year: ");
    let current_year = Utc::now().year() as i32;

    match calculate_age(birth_year, current_year) {
        Ok(age) => display_result(age),
        Err(err) => println!("Error: {}", err),
    }
}

fn get_user_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn calculate_age(birth_year: i32, current_year: i32) -> Result<Age, &'static str> {
    const SECONDS_IN_MINUTE: i32 = 60;
    const MINUTES_IN_HOUR: i32 = 60;
    const HOURS_IN_DAY: i32 = 24;
    const DAYS_IN_MONTH: i32 = 30; // Assuming an average month has 30 days
    const MONTHS_IN_YEAR: i32 = 12;

    if birth_year > current_year {
        return Err("Birth year cannot be greater than the current year.");
    }

    let age_in_years = current_year - birth_year;
    let age_in_months = age_in_years * MONTHS_IN_YEAR;
    let age_in_days = age_in_years * DAYS_IN_MONTH * MONTHS_IN_YEAR;
    let age_in_hours = age_in_days * HOURS_IN_DAY;
    let age_in_seconds = age_in_hours * MINUTES_IN_HOUR * SECONDS_IN_MINUTE;

    Ok(Age {
        years: age_in_years,
        months: age_in_months,
        days: age_in_days,
        hours: age_in_hours,
        seconds: age_in_seconds,
    })
}

fn display_result(age: Age) {
    println!("Your age is:");
    println!("Years: {}", age.years);
    println!("Months: {}", age.months);
    println!("Days: {}", age.days);
    println!("Hours: {}", age.hours);
    println!("Seconds: {}", age.seconds);
}
