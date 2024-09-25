use std::cmp::Ordering;
use std::collections::HashMap;

pub fn calculate_median_of_the_data(data: &[i32]) {
    let part: Option<_> = partition_data(data);
    println!("Partitioned data is: {:?}", part);

    let selected: Option<_> = select_data(data, 5);
    println!("Selection at ordered index {} is {:?}", 5, selected);

    let median: Option<f32> = calculate_median(data);
    println!("Median is {:?}", median);
}

pub fn calculate_mode_of_the_data(data: &[i32]) {
    let frequencies: HashMap<&i32, i32> = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is: {:?}", mode)
}

pub fn calculate_zscore(data: &[i32]) {
    let data_mean: Option<f32> = calculate_mean(data);
    println!("Mean of the data is: {:?}", data_mean);

    let data_std_deviation: Option<f32> = calculate_deviation(data);
    println!("Standard deviation is {:?}", data_std_deviation);

    let zscore: Option<f32> = match (data_mean, data_std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff: f32 = data[4] as f32 - mean;
            Some(diff / std_deviation)
        }
        _ => None,
    };

    println!(
        "Z-score of data at index 4 (with value {}) is {:?}",
        data[4], zscore
    );
}

// Auxiliary functions
fn partition_data(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot: i32 = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });
            Some((left, pivot, right))
        }
    }
}

fn select_data(data: &[i32], k: usize) -> Option<f32> {
    let partitioned_data: Option<_> = partition_data(data);

    match partitioned_data {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx: usize = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot as f32),
                Ordering::Greater => select_data(&left, k),
                Ordering::Less => select_data(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn calculate_median(data: &[i32]) -> Option<f32> {
    let size: usize = data.len();
    match size {
        even if even % 2 == 0 => {
            let first_median: Option<f32> = select_data(data, (even / 2) - 1);
            let second_median: Option<f32> = select_data(data, even / 2);

            match (first_median, second_median) {
                (Some(first), Some(secord)) => Some((first + secord) / 2.0),
                _ => None,
            }
        }
        odd => select_data(data, odd / 2),
    }
}

fn calculate_mean(data: &[i32]) -> Option<f32> {
    let sum: f32 = data.iter().sum::<i32>() as f32;
    let count: usize = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn calculate_deviation(data: &[i32]) -> Option<f32> {
    match (calculate_mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}
