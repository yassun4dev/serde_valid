use serde_json::json;
use serde_valid::Validate;

#[test]
fn custom_is_ok_test() {
    fn user_validation(_val: &Vec<i32>) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_is_err_test() {
    fn user_validation(_val: &Vec<i32>) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "this is custom message.".to_string(),
        ))
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}
