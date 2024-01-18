#[allow(dead_code)]
#[derive(Debug)]

// Define a struct representing a person
struct Person {
    // Struct fields
    name: String,
    age: u32,
}

// Implementation block for the Person struct
impl Person {
    // Method to create a new person
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // Method to get person's details
    fn get_details(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }
}

// Function to greet a person
fn greet(person: &Person) {
    println!("Hello, {}!", person.get_details());
}

// Enum representing days of the week
#[derive(Debug)]
enum DaysOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

// Trait representing a shape
trait Shape {
    // Method to calculate area
    fn calculate_area(&self) -> f64;
}

// Struct representing a circle implementing the Shape trait
struct Circle {
    radius: f64,
}

// Implementation block for the Circle struct
impl Circle {
    // Method to create a new circle
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

// Implementation of the Shape trait for Circle
impl Shape for Circle {
    // Implementation of the calculate_area method
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

// Type alias for a function that takes two f64 values and returns an f64
type MathOperation = fn(f64, f64) -> f64;

// Function to perform a math operation
fn perform_math_operation(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    // Create a person
    let john = Person::new("John Doe", 25);

    // Greet the person
    greet(&john);

    // Use the DaysOfWeek enum
    let today = DaysOfWeek::Wednesday;
    println!("Today is {:?}", today);

    // Create a circle
    let my_circle = Circle::new(5.0);
    println!("The area of the circle is: {}", my_circle.calculate_area());

    // Use the MathOperation type alias
    let result = perform_math_operation(10.0, 20.0);
    println!("Result of the math operation: {}", result);
}
