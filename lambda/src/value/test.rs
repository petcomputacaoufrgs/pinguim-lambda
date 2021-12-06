use super::NestedValue;
use super::Value;

#[test]
fn not_beta_equiv_by_var_both_bound() {
    // λx. λy. x x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("x"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. y x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("y"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    assert!(!left.beta_equiv(&right));
}

#[test]
fn not_beta_equiv_by_structure() {
    // λx. λy. x x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("x"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. (λy. y) x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Application {
            function: NestedValue::new(Value::Lambda {
                parameter: String::from("y"),
                body: NestedValue::new(Value::Variable(String::from("y"))),
            }),
            argument: NestedValue::new(Value::Variable(String::from("x"))),
        }),
    };

    assert!(!left.beta_equiv(&right));
}

#[test]
fn not_beta_equiv_by_var_both_free() {
    // λx. λy. a x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("a"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. b x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("b"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    assert!(!left.beta_equiv(&right));
}

#[test]
fn not_beta_equiv_by_var_free_and_bound() {
    // λx. λy. x x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("x"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. b x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("y"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("b"))),
                argument: NestedValue::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    assert!(!left.beta_equiv(&right));
}

#[test]
fn beta_equiv_identical() {
    // λn. λm. λf. λx. n f (m f x)
    let left = Value::Lambda {
        parameter: String::from("n"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("m"),
            body: NestedValue::new(Value::Lambda {
                parameter: String::from("f"),
                body: NestedValue::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Variable(
                                String::from("n"),
                            )),
                            argument: NestedValue::new(Value::Variable(
                                String::from("f"),
                            )),
                        }),
                        argument: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Application {
                                function: NestedValue::new(Value::Variable(
                                    String::from("m"),
                                )),
                                argument: NestedValue::new(Value::Variable(
                                    String::from("f"),
                                )),
                            }),
                            argument: NestedValue::new(Value::Variable(
                                String::from("x"),
                            )),
                        }),
                    }),
                }),
            }),
        }),
    };

    // λn. λm. λf. λx. n f (m f x)
    let right = left.clone();

    assert!(left.beta_equiv(&right));
}

#[test]
fn beta_equiv_different_vars() {
    // λn. λm. λf. λx. n f (m f x)
    let left = Value::Lambda {
        parameter: String::from("n"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("m"),
            body: NestedValue::new(Value::Lambda {
                parameter: String::from("f"),
                body: NestedValue::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Variable(
                                String::from("n"),
                            )),
                            argument: NestedValue::new(Value::Variable(
                                String::from("f"),
                            )),
                        }),
                        argument: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Application {
                                function: NestedValue::new(Value::Variable(
                                    String::from("m"),
                                )),
                                argument: NestedValue::new(Value::Variable(
                                    String::from("f"),
                                )),
                            }),
                            argument: NestedValue::new(Value::Variable(
                                String::from("x"),
                            )),
                        }),
                    }),
                }),
            }),
        }),
    };

    // λm. λn. λs. λz. m s (n s z)
    let right = Value::Lambda {
        parameter: String::from("m"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("n"),
            body: NestedValue::new(Value::Lambda {
                parameter: String::from("s"),
                body: NestedValue::new(Value::Lambda {
                    parameter: String::from("z"),
                    body: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Variable(
                                String::from("m"),
                            )),
                            argument: NestedValue::new(Value::Variable(
                                String::from("s"),
                            )),
                        }),
                        argument: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Application {
                                function: NestedValue::new(Value::Variable(
                                    String::from("n"),
                                )),
                                argument: NestedValue::new(Value::Variable(
                                    String::from("s"),
                                )),
                            }),
                            argument: NestedValue::new(Value::Variable(
                                String::from("z"),
                            )),
                        }),
                    }),
                }),
            }),
        }),
    };

    assert!(left.beta_equiv(&right));
}
