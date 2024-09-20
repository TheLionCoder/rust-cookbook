mod character_set;

fn main() {
    character_set::encode_string().unwrap();
    character_set::encode_url();
}
