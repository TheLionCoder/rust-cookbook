use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

pub fn get_access() {
    let access: Option<&Vec<&str>> = PRIVILEGES.get("James");
    print!("James: {:?}", access);

    show_access("Jim")
}

fn show_access(name: &str) {
    let access: Option<&Vec<&str>> = PRIVILEGES.get("name");
    println!("{}: {:?}", name, access);
}
