use serde_json::json;
use serde_valid::Validate;

#[test]
fn enum_named_variant_validation_is_ok() {
    fn ok_rule(_a: &TestStruct, _b: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }
    #[derive(Validate)]
    enum TestEnum {
        #[rule(ok_rule(a, b))]
        Named {
            #[validate]
            a: TestStruct,
            #[validate]
            b: TestStruct,
        },
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::Named {
        a: TestStruct { val: 5 },
        b: TestStruct { val: 5 },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enum_unnamed_variant_validation_is_ok() {
    #[derive(Validate)]
    enum TestEnum {
        UnNamed(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
            #[validate] TestStruct,
        ),
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::UnNamed(5, TestStruct { val: 5 });
    assert!(s.validate().is_ok());
}

#[test]
fn enum_newtype_variant_validation_is_ok() {
    #[derive(Validate)]
    enum TestEnum {
        NewType(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
        ),
    }

    let s = TestEnum::NewType(15);
    assert!(s.validate().is_err());
}

#[test]
fn enum_named_variant_validation_is_err() {
    fn err_rule(_a: &TestStruct, _b: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Rule error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    enum TestEnum {
        #[rule(err_rule(a, b))]
        Named {
            #[validate]
            a: TestStruct,
            #[validate]
            b: TestStruct,
        },
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::Named {
        a: TestStruct { val: 12 },
        b: TestStruct { val: 12 },
    };
    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": ["Rule error."],
            "properties": {
                "a": {
                    "errors": [],
                    "properties": {
                        "val": {
                            "errors": ["The number must be `<= 10`."]
                        }
                    }
                },
                "b": {
                    "errors": [],
                    "properties": {
                        "val": {
                            "errors": ["The number must be `<= 10`."]
                        }
                    }
                }
            }
        })
        .to_string()
    );
}

#[test]
fn enum_unnamed_variant_validation_is_err() {
    fn err_rule(_a: &TestStruct, _b: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Rule error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    enum TestEnum {
        #[rule(err_rule(0, 1))]
        Named(#[validate] TestStruct, #[validate] TestStruct),
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::Named(TestStruct { val: 12 }, TestStruct { val: 12 });

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": ["Rule error."],
            "items": {
                "0": {
                    "errors": [],
                    "properties": {
                        "val": {
                            "errors": ["The number must be `<= 10`."]
                        }
                    }
                },
                "1": {
                    "errors": [],
                    "properties": {
                        "val": {
                            "errors": ["The number must be `<= 10`."]
                        }
                    }
                }
            }
        })
        .to_string()
    );
}

#[test]
fn enum_newtype_variant_validation_is_err() {
    fn err_rule(_a: &u32) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Rule error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    enum TestEnum {
        #[rule(err_rule(0))]
        NewType(#[validate(minimum = 5)] u32),
    }

    let s = TestEnum::NewType(4);

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": ["Rule error.", "The number must be `>= 5`."]
        })
        .to_string()
    );
}
