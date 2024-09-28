mod downloads;

fn main() {
    downloads::download_file_to_tmp_dir().unwrap();
    downloads::post_file_to_paste_rs().unwrap();
    downloads::make_a_partial_download_with_http().unwrap();
}
