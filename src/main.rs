#[derive(PartialEq, Clone)]
struct Person {
    name: String,
    gender: Sex,
    marital_status: MaritalStatus,
}

impl Person {
    fn new(name: &str, gender: Sex, marital_status: MaritalStatus) -> Self {
        Self {
            name: name.to_string(),
            gender,
            marital_status,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_gender(&self) -> String {
        match self.gender {
            Sex::Female => "Female".into(),
            Sex::Male => "Male".into(),
        }
    }

    fn get_marital_status(&self) -> String {
        match self.marital_status {
            MaritalStatus::Married => "Married".into(),
            MaritalStatus::Single => "Single".into(),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Sex {
    Female,
    Male,
}

#[derive(PartialEq, Clone)]
enum MaritalStatus {
    Married,
    Single,
}

enum Criteria {
    Female,
    Male,
    Single,
    And(Box<Criteria>, Box<Criteria>),
    Or(Box<Criteria>, Box<Criteria>),
}

impl Criteria {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        match &self {
            Criteria::Female => self.meet_criteria_female(persons),
            Criteria::Male => self.meet_criteria_male(persons),
            Criteria::Single => self.meet_criteria_single(persons),
            Criteria::And(first, second) => self.meet_criteria_and(persons, &first, &second),
            Criteria::Or(first, second) => self.meet_criteria_or(persons, first, second),
        }
    }
    fn meet_criteria_female(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut females: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let Sex::Female = person.gender {
                females.push(person.clone())
            }
        }
        females
    }
    fn meet_criteria_male(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut males: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let Sex::Male = person.gender {
                males.push(person.clone())
            }
        }
        males
    }
    fn meet_criteria_single(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut singletons: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let MaritalStatus::Single = person.marital_status {
                singletons.push(person.clone())
            }
        }
        singletons
    }
    fn meet_criteria_and(
        &self,
        persons: &Vec<Person>,
        first: &Box<Criteria>,
        second: &Box<Criteria>,
    ) -> Vec<Person> {
        let first_criteria_persons: Vec<Person> = first.meet_criteria(persons);
        let second_criteria_persons: Vec<Person> = second.meet_criteria(&first_criteria_persons);
        second_criteria_persons
    }
    fn meet_criteria_or(
        &self,
        persons: &Vec<Person>,
        first: &Box<Criteria>,
        second: &Box<Criteria>,
    ) -> Vec<Person> {
        let mut first_criteria_persons: Vec<Person> = first.meet_criteria(persons);
        let second_criteria_persons: Vec<Person> = second.meet_criteria(persons);
        for person in second_criteria_persons.iter() {
            if !first_criteria_persons.contains(person) {
                first_criteria_persons.push(person.clone())
            }
        }
        first_criteria_persons
    }
}

fn print_persons(persons: Vec<Person>) {
    for person in persons.iter() {
	println!("Name: {}, Gender: {}, Marital Status: {}", person.get_name(), person.get_gender(), person.get_marital_status());
    }
}

fn main() {
    let mut persons: Vec<Person> = Vec::new();
    persons.push(Person::new("Robert", Sex::Male, MaritalStatus::Single));
    persons.push(Person::new("John", Sex::Male, MaritalStatus::Married));
    persons.push(Person::new("Laura", Sex::Female, MaritalStatus::Married));
    persons.push(Person::new("Diana", Sex::Female, MaritalStatus::Single));
    persons.push(Person::new("Mike", Sex::Male, MaritalStatus::Single));
    persons.push(Person::new("Bobby", Sex::Male, MaritalStatus::Single));

    let male: Criteria = Criteria::Male;
    let female: Criteria = Criteria::Female;
    let single: Criteria = Criteria::Single;
    let single_male: Criteria = Criteria::And(Box::new(Criteria::Male), Box::new(Criteria::Single));
    let single_or_female: Criteria = Criteria::Or(Box::new(Criteria::Female), Box::new(Criteria::Single));

    println!("Males: ");
    print_persons(male.meet_criteria(&persons));
    println!("\nFemales: ");
    print_persons(female.meet_criteria(&persons));
    println!("\nSingle people: ");
    print_persons(single.meet_criteria(&persons));
    println!("\nSingle Males: ");
    print_persons(single_male.meet_criteria(&persons));
    println!("\nSingle or Female: ");
    print_persons(single_or_female.meet_criteria(&persons));

}
