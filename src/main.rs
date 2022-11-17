mod custom_error;
mod database;
mod convert;

use std::vec::Vec;

use database::solution;
use convert::convert_str_input_to_string_input;

fn main() {
    let input: Vec<Vec<&str>> = vec![
        vec!["set", "table1", "A", "B", "key1", "value1", "C", "D"],
        vec!["set", "table2", "key2", "value2", "V", "W", "X", "Y", "Z", "A"],
        vec!["get", "table2", "X"],
        vec!["get", "table2", "Z"],
        vec!["get", "table1", "key1"],
        vec!["get", "table1", "C"],
        vec!["delete", "table1", "key1"],
        vec!["get", "table2", "key2"],
        vec!["delete", "table2", "key2"],
        vec!["get", "table2", "key2"],
        vec!["delete", "table2", "P"],
        vec!["get", "table2", "P"]
    ];

    run_solution(input);

    let input = vec![];
    run_solution(input);
}



// This function is for printing the solution. It is not the real
// subject under test. See the solution function instead.
fn run_solution(input: Vec<Vec<&str>>) {
    let v = convert_str_input_to_string_input(input);
    match solution(v) {
        Err(error) => panic!("{:#?}", error),
        Ok(results) => {
            println!("[");
            for result in results {
                println!("{},", if result.len() == 0 { "\"\""} else { &result });
            }
            println!("]");
        }
    }
}
