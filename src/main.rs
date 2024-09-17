mod parallel_tasks;

fn main() {
    parallel_tasks::mutate_elements_in_parallel();
    parallel_tasks::match_patterns_in_parallel();
    parallel_tasks::search_items_given_predicate_parallel();
    parallel_tasks::sort_vector_in_parallel();
    parallel_tasks::map_reduce_in_parallel();
}
