use std::collections::HashMap;
use std::option::Option;
use std::result::Result as BaseResult;
// use std::ops::FnOnce;

use super::custom_error::Error;

type Result<T> = BaseResult<T, Error>;

// This is the subject under test. The code exercise provided a
// slightly different signature
// fn solution(queries: Vec<Vec<String>>) -> Vec<String>
// In that case, I was attempting to embed user errors in the
// results. Most real world applications tend to throw exceptions
// or panic in such situations, and I have chosen to do this
// here instead. See tests/database.rs for tests.
pub fn solution(queries: Vec<Vec<String>>) -> Result<Vec<String>> {
    if queries.len() < 1 {
        return Err(Error::new("No queries were supplied".to_string()));
    }

    if queries.len() > 250 {
        return Err(Error::new("More than 250 queries are not supported.".to_string()));
    }

    validate(&queries)?;
    let mut tables: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut results: Vec<String> = vec![];
    for query in queries {
        let result = match query[0].as_str().to_uppercase().as_str() {
            "GET" => get(query[2].as_str(), tables.get(&query[1])),
            "DELETE" => delete(query[2].as_str(), tables.get_mut(&query[1])),
            "SET" => set(&query[2..query.len()], query[1].as_str(), &mut tables),
            _ => panic!("An internal error occurred - please contact XYZ")
        };
        results.push(result);
    }

    Ok(results)
}

fn set(parameters: &[String], table: &str, tables: &mut HashMap<String, HashMap<String, String>>) -> String {
    let rows = tables.entry(table.to_string()).or_insert_with(|| { HashMap::new() });
    let mut i = 0;
    while i < parameters.len() {
        rows.entry(parameters[i].as_str().to_string())
            .and_modify(|val| { *val = parameters[i + 1].as_str().to_string() })
            .or_insert(parameters[i + 1].as_str().to_string());
        i = i + 2;
    }

    String::from("")
}

fn get(key: &str, table_entry: Option<&HashMap<String, String>>) -> String {
    match table_entry {
        None => String::from(""),
        Some(rows) => match rows.get(key) {
            None => String::from(""),
            Some(value) => value.to_string()
        }
    }
}

fn delete(key: &str, table_entry: Option<&mut HashMap<String, String>>) -> String {
    match table_entry {
        None => String::from("false"),
        Some(rows) => match rows.remove(key) {
            None => String::from("false"),
            Some(_value) => String::from("true")
        }
    }
}

fn validate(queries: &Vec<Vec<String>>) -> Result<()> {
    let mut i = 0;
    let mut errors: Vec<String> = vec![];
    let validate_get_delete = | errors: &mut Vec<String>, query: &Vec<String>, op, line | {
        if query.len() != 3 {
            errors.push(format!("At line {}, {} expects 2 arguments - a table name and a key", line, op));
        }
    };

    for query in queries {
        i = i + 1;
        if query.len() < 1 {
            errors.push(format!("No operation was specified at line {}. Expected GET, SET or DELETE", i));
        } else {
            match query[0].as_str().to_uppercase().as_str() {
                "GET" => validate_get_delete(&mut errors, &query, "GET", i),
                "DELETE" => validate_get_delete(&mut errors, &query, "DELETE", i),
                "SET" => {
                    if query.len() < 4 {
                        errors.push(format!("At line {}, SET expects a table and at least one key and one value", i));
                    } else if query.len() % 2 == 1 {
                        errors.push(format!("At line {}, SET was not provided the value for key {}", i, query[query.len() - 1]));
                    }
                },
                _ => errors.push(format!("At line {}, an unknown operation {} was supplied", i, query[0]))
            }
        }
    }

    if errors.len() > 0 {
        return Err(Error::new(errors.as_slice().join("\n")));
    }

    Ok(())
}

include!("database/test.rs");
