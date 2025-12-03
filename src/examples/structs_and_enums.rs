/*
    Structs and Enums Exercise
struct Employee {
    name: String,
    age: u8,
    email_id: String,
    experience: u8,
    location: Location,
}
impl Employee {
    fn new(name: String) -> Employee {
        let name_ref = name.clone();
        Employee {
            name: name,
            age: 25,
            email_id: String::from(name_ref + "@company.com"),
            experience: 0,
            location: Location::India(String::from("Hyderabad")),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    fn set_email_id(&mut self, email_id: String) {
        self.email_id = email_id;
    }
    fn get_location(&self) -> &Location {
        &self.location
    }
    fn get_location_city(&self) -> &str {
        match &self.location {
            Location::India(city) => city,
            Location::US(city) => city,
            Location::UK(city) => city,
        }
    }
    fn print_employee(&self) {
        println!(
            "Employee:\nname = {}, \nage = {}, \nemail_id = {}, \nexperience = {}, \nlocation = {:?}, city = {}",
            self.get_name(),
            self.age,
            self.email_id,
            self.experience,
            self.get_location(),
            self.get_location_city()
        );
        println!("---------------------------");
    }
    fn set_experience(&mut self, experience: u8) {
        self.experience = experience;
    }
}
#[derive(Debug)]
enum Location {
    India(String),
    US(String),
    UK(String),
}

    impl Location {
    fn city(&self) -> &str {
        match self {
            Location::India(city) => city,
            Location::US(city) => city,
            Location::UK(city) => city,
        }
    }
}

    let mut employee1 = Employee::new(String::from("Vijay Singh"));
    employee1.set_email_id(String::from("vijahsingh@company.com"));

    let mut employee2 = Employee::new(String::from("Anjali Sharma"));
    employee2.set_email_id(String::from("anjaliSharma@company.com"));
    employee2.location = Location::US(String::from("New York"));

    let mut employee3 = Employee::new(String::from("Prateek Gupta"));
    employee3.set_email_id(String::from("prateek@company.com"));
    employee3.location = Location::UK(String::from("London"));

    let mut employee4 = Employee::new(String::from("Rohit Verma"));
    employee4.set_email_id(String::from("rohit.verma@company.com"));
    employee4.location = Location::India(String::from("Bangalore"));

    employee1.set_age(61);
    employee2.set_age(28);
    employee2.set_experience(5);

    employee1.print_employee();
    employee2.print_employee();
    employee3.print_employee();
    employee4.print_employee();

*/
