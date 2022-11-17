#[cfg(test)]
mod tests {
    use crate::solution;
    use crate::custom_error::Error;

    #[test]
    fn happy_path() {
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
        let expected_result_length = input.len();
        let real_input = crate::convert::convert_str_input_to_string_input(input);
        let results = solution(real_input).unwrap();
        assert_eq!(0, results[0].len());
        assert_eq!(0, results[1].len());
        assert_eq!("Y", results[2]);
        assert_eq!("A", results[3]);
        assert_eq!("value1", results[4]);
        assert_eq!("D", results[5]);
        assert_eq!("true", results[6]);
        assert_eq!("value2", results[7]);
        assert_eq!("true", results[8]);
        assert_eq!(0, results[9].len());
        assert_eq!("false", results[10]);
        assert_eq!(0, results[11].len());
        assert_eq!(expected_result_length, results.len());
    }

    #[test]
    fn more_complicated_operations() {
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
            vec!["get", "table2", "P"],
            // Add back a value that was deleted, replace a value of an existing key, and add a brand new key and value
            vec!["set", "table2", "P", "NewValue", "V", "ReplaceValue", "NewKey", "AnotherNewValue"],
            vec!["get", "table2", "P"],
            vec!["get", "table2", "V"],
            vec!["get", "table2", "NewKey"],
            vec!["get", "table2", "Z"] // an unchanged key
        ];
        let expected_result_length = input.len();
        let real_input = crate::convert::convert_str_input_to_string_input(input);
        let results = solution(real_input).unwrap();
        assert_eq!(0, results[12].len());
        assert_eq!("NewValue", results[13]);
        assert_eq!("ReplaceValue", results[14]);
        assert_eq!("AnotherNewValue", results[15]);
        assert_eq!("A", results[16]);
        assert_eq!(expected_result_length, results.len());
    }

    #[test]
    fn when_there_are_input_errors_in_several_lines_then_all_errors_should_be_returned_together() {
        let input: Vec<Vec<&str>> = vec![
            vec!["set", "table1", "A", "B", "key1", "value1", "wrong_key_without_value"],
            vec!["wrong", "table2", "key2", "value2", "V", "W", "X", "Y", "Z", "A"],
            vec!["get", "table2", "X"],
            vec!["get", "table2", "Z"],
            vec![],
            vec!["get", "table1", "C"],
            vec!["delete", "table1", "key1"],
            vec!["get", "table2", "key2"],
            vec!["delete", "table2", "key2"],
            vec!["get", "table2", "key2"],
            vec!["delete", "table2", "P", "wrong_extra"],
            vec!["get", "table2", "P"]
        ];
        let real_input = crate::convert::convert_str_input_to_string_input(input);
        let error = solution(real_input).unwrap_err();
        let expected_message = 
r###"At line 1, SET was not provided the value for key wrong_key_without_value
At line 2, an unknown operation wrong was supplied
No operation was specified at line 5. Expected GET, SET or DELETE
At line 11, DELETE expects 2 arguments - a table name and a key"###;
        let expected = Error::new(expected_message.to_string());
        assert_eq!(expected, error);
    }

    #[test]
    fn when_an_empty_input_is_supplied_then_an_error_with_message_should_be_returned() {
        let error = solution(vec![]).unwrap_err();
        let expected = Error::new("No queries were supplied".to_string());
        assert_eq!(expected, error);
    }

    #[test]
    fn when_more_than_250_queries_are_supplied_then_an_error_with_message_should_be_returned() {
        let mut input = Vec::with_capacity(251);
        for _i in 1..252 {
            input.push(vec![]);
        }
        
        let error = solution(input).unwrap_err();
        let expected = Error::new("More than 250 queries are not supported.".to_string());
        assert_eq!(expected, error);
    }
}
