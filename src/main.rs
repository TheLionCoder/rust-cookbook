mod tarballs;

fn main() {
    // random values
/*    random_values::generate_random_integers();
    random_values::generate_random_range_numbers();
    random_values::generate_distributed_random_numbers().unwrap();
    random_values::generate_custom_random_numbers();
    random_values::create_random_password_alphanumeric();
    random_values::create_random_password_user_defined();*/

    // sort vectors
    //sorting_vectors::sort_structs_vector();

    // cli
    // parse_cli::parse_cli();
    //ansi_terminal::print_styled_text();

    // tarballs
    tarballs::decompress_tarball().unwrap();
    tarballs::compress_tarball().unwrap();
    tarballs::decompress_tarball_remove_prefix().unwrap();


}
