mod statistics;

fn main() {
    let data: [i32; 10] = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
    statistics::calculate_median_of_the_data(&data);
    statistics::calculate_mode_of_the_data(&data);
    statistics::calculate_zscore(&data);
}
