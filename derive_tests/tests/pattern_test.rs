use serde_valid::Validate;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

#[test]
fn pattern_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a str,
    }

    let s = TestStruct { val: "2020-09-10" };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_cow_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Cow<'a, str>,
    }

    let s = TestStruct {
        val: Cow::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_os_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a OsStr,
    }

    let s = TestStruct {
        val: OsStr::new("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_os_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: OsString,
    }

    let s = TestStruct {
        val: OsString::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_path_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_path_buf_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("2020/09/10"),
    };
    assert!(s.validate().is_err());
}

#[test]
fn pattern_array_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("2020-09-10"), String::from("2020-10-10")],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_nested_array_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Vec<Vec<String>>,
    }

    let s = TestStruct {
        val: vec![
            vec![String::from("2020-09-10"), String::from("2020-10-10")],
            vec![String::from("2020-11-10"), String::from("2020-12-10")],
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Option<String>,
    }

    let s = TestStruct {
        val: Some(String::from("2020-09-10")),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_nested_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Option<Option<String>>,
    }

    let s = TestStruct {
        val: Some(Some(String::from("2020-09-10"))),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_array_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Vec<Option<String>>,
    }

    let s = TestStruct {
        val: vec![
            Some(String::from("2020-09-10")),
            Some(String::from("2020-10-10")),
            None,
        ],
    };
    assert!(s.validate().is_ok());
}