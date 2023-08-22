struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phone: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phone: String::from("987-654-3210"),
        id: 2,
    });

    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phone: String::from("555-555-5555"),
        id: 3,
    });

    students.push(Student {
        name: String::from("David"),
        email: String::from("david@example.com"),
        phone: String::from("111-222-3333"),
        id: 4,
    });

    students.push(Student {
        name: String::from("Eve"),
        email: String::from("eve@example.com"),
        phone: String::from("999-888-7777"),
        id: 5,
    });

    
    let index = 1;
    match students.get(index) {
        Some(student) => {
            println!("Student at index {}:", index);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone: {}", student.phone);
            println!("ID: {}", student.id);
        }
        None => {
            println!("Student at index {} not found.", index);
        }
    }
}
   
