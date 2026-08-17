use fancy_regex::Regex;

#[cfg(test)]
mod split_tests {
    use super::*;

    fn split_to_vec<'a>(re_str: &'a str, target: &'a str) -> Vec<&'a str> {
        let re = Regex::new(re_str).unwrap();
        re.split(target).map(|x| x.unwrap()).collect()
    }

    #[test]
    fn split_left() {
        let result: Vec<&str> = split_to_vec("1", "123");
        assert_eq!(result, vec!["", "23"]);
    }

    #[test]
    fn split_center() {
        let result: Vec<&str> = split_to_vec("2", "123");
        assert_eq!(result, vec!["1", "3"]);
    }

    #[test]
    fn split_right() {
        let result: Vec<&str> = split_to_vec("3", "123");
        assert_eq!(result, vec!["12", ""]);
    }

    #[test]
    fn split_no_matches() {
        let result: Vec<&str> = split_to_vec("4", "123");
        assert_eq!(result, vec!["123"]);
    }

    #[test]
    fn split_empty() {
        let result: Vec<&str> = split_to_vec("1", "");
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn split_by_empty() {
        let result: Vec<&str> = split_to_vec("", "123");
        assert_eq!(result, vec!["", "1", "2", "3", ""]);
    }

    #[test]
    fn split_by_own() {
        let result: Vec<&str> = split_to_vec("123", "123");
        assert_eq!(result, vec!["", ""]);
    }

    #[test]
    fn split_consecutive_matches() {
        let result: Vec<&str> = split_to_vec("1", "111");
        assert_eq!(result, vec!["", "", "", ""]);
    }

    #[test]
    fn split_by_substring() {
        let result: Vec<&str> = split_to_vec("123", "123456");
        assert_eq!(result, vec!["", "456"]);

        let result: Vec<&str> = split_to_vec("234|678", "123456789");
        assert_eq!(result, vec!["1", "5", "9"]);
    }

    #[test]
    fn split_multiple_different_characters() {
        let result: Vec<&str> = split_to_vec("[1-3]", "123456");
        assert_eq!(result, vec!["", "", "", "456"]);
    }

    #[test]
    fn split_mixed_characters() {
        let result: Vec<&str> = split_to_vec("[236]", "123456");
        assert_eq!(result, vec!["1", "", "45", ""]);
    }

    #[test]
    fn split_with_backreferences() {
        let result: Vec<&str> = split_to_vec(r"(1|2)\1", "12112122");
        assert_eq!(result, vec!["12", "21", ""]);
    }

    #[test]
    fn split_with_look_around() {
        let result: Vec<&str> = split_to_vec(r"(?<=1)2", "12112122");
        assert_eq!(result, vec!["1", "11", "1", "2"]);

        let result: Vec<&str> = split_to_vec(r"1(?=2)", "12112122");
        assert_eq!(result, vec!["", "21", "2", "22"]);

        let result: Vec<&str> = split_to_vec(r"(?<=2)1(?=2)", "12112122");
        assert_eq!(result, vec!["12112", "22"]);
    }
}

#[cfg(test)]
mod splitn_tests {
    use super::*;

    fn splitn_to_vec<'a>(re_str: &'a str, target: &'a str, limit: usize) -> Vec<&'a str> {
        let re = Regex::new(re_str).unwrap();
        re.splitn(target, limit).map(|x| x.unwrap()).collect()
    }

    #[test]
    fn splitn_limit_lt_num_mathes() {
        let splitn_test_cases = [
            ("1", "123", vec!["", "23"]),
            ("2", "123", vec!["1", "3"]),
            ("3", "123", vec!["12", ""]),
            ("", "123", vec!["", "1", "2", "3", ""]),
            ("1", "", vec![""]),
        ];

        for (re_str, target, expected) in splitn_test_cases {
            let result: Vec<&str> = splitn_to_vec(re_str, target, 6);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn splitn_limit_eq_num_mathes() {
        let splitn_test_cases = [
            ("1", "123", vec!["", "23"]),
            ("2", "123", vec!["1", "3"]),
            ("3", "123", vec!["12", ""]),
            ("", "123", vec!["", "1", "2", "3", ""]),
            ("1", "", vec![""]),
        ];

        for (re_str, target, expected) in splitn_test_cases {
            let result: Vec<&str> = splitn_to_vec(re_str, target, expected.len());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn splitn_limit_st_num_mathes() {
        let splitn_test_cases = [
            ("1", "123", vec!["123"]),
            ("2", "123", vec!["123"]),
            ("3", "123", vec!["123"]),
            ("", "123", vec!["123"]),
        ];

        for (re_str, target, expected) in splitn_test_cases {
            let result: Vec<&str> = splitn_to_vec(re_str, target, 1);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn splitn_limit_zero() {
        let vec_empty: Vec<&str> = Vec::new();
        let splitn_test_cases = [
            ("1", "123", &vec_empty),
            ("2", "123", &vec_empty),
            ("3", "123", &vec_empty),
            ("", "123", &vec_empty),
            ("1", "", &vec_empty),
        ];

        for (re_str, target, expected) in splitn_test_cases {
            let result: Vec<&str> = splitn_to_vec(re_str, target, 0);
            assert_eq!(&result, expected);
        }
    }
}
