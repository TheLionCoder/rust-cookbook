#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age
        }
    }
}

pub(crate) fn sort_structs_vector(){
    let mut people: Vec<Person> = vec![
        Person::new(String::from("Goku"), 30),
        Person::new(String::from("Vegeta"), 35),
        Person::new(String::from("Piccolo"), 25)
    ];

    // sort by natural order (Name and age
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Goku".to_string(), 30),
            Person::new("Piccolo".to_string(), 25),
            Person::new("Vegeta".to_string(), 35)
        ]
    );

    // sort by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new(String::from("Vegeta"), 35),
            Person::new(String::from("Goku"), 30),
            Person::new(String::from("Piccolo"), 25)
        ]
    )
}