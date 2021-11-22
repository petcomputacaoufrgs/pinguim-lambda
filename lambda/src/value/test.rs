use super::Value;

#[test]
fn capture() {
    //λa. (λx. λa. x a) a
    let mut input_value = Value::Lambda {
        parameter: String::from("a"),
        body: Box::new(Value::Application {
            function: Box::new(Value::Lambda {
                parameter: String::from("x"),
                body: Box::new(Value::Lambda {
                    parameter: String::from("a"),
                    body: Box::new(Value::Application {
                        function: Box::new(Value::Variable(String::from("x"))),
                        argument: Box::new(Value::Variable(String::from("a"))),
                    }),
                }),
            }),
            argument: Box::new(Value::Variable(String::from("a"))),
        }),
    };

    //λa. (λa_. a a_)
    let output_value = Value::Lambda {
        parameter: String::from("a"),
        body: Box::new(Value::Lambda {
            parameter: String::from("a_"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("a"))),
                argument: Box::new(Value::Variable(String::from("a_"))),
            }),
        }),
    };

    input_value.reduce();
    assert_eq!(input_value, output_value);
}

#[test]
fn rename_capture() {
    // λa_. λa. (λx. λa. x a a_) a
    let mut input_value = Value::Lambda {
        parameter: String::from("a_"),
        body: Box::new(Value::Lambda {
            parameter: String::from("a"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: Box::new(Value::Lambda {
                        parameter: String::from("a"),
                        body: Box::new(Value::Application {
                            function: Box::new(Value::Application {
                                function: Box::new(Value::Variable(
                                    String::from("x"),
                                )),
                                argument: Box::new(Value::Variable(
                                    String::from("a"),
                                )),
                            }),
                            argument: Box::new(Value::Variable(String::from(
                                "a_",
                            ))),
                        }),
                    }),
                }),
                argument: Box::new(Value::Variable(String::from("a"))),
            }),
        }),
    };

    // λa_. λa. (λa__. a a__ a_)
    let output_value = Value::Lambda {
        parameter: String::from("a_"),
        body: Box::new(Value::Lambda {
            parameter: String::from("a"),
            body: Box::new(Value::Lambda {
                parameter: String::from("a__"),
                body: Box::new(Value::Application {
                    function: Box::new(Value::Application {
                        function: Box::new(Value::Variable(String::from("a"))),
                        argument: Box::new(Value::Variable(String::from(
                            "a__",
                        ))),
                    }),
                    argument: Box::new(Value::Variable(String::from("a_"))),
                }),
            }),
        }),
    };

    input_value.reduce();
    assert_eq!(input_value, output_value);
}

#[test]
fn two_power_three() {
    // (λf. λx. f (f (f x))) (λf. λx. f (f x))
    let mut input_value = Value::Application {
        function: Box::new(Value::church_numeral(3)),
        argument: Box::new(Value::church_numeral(2)),
    };

    // λf. λx. f (f (f (f (f (f (f (f x)))))))
    let output_value = Value::church_numeral(8);

    input_value.reduce();
    assert!(input_value.beta_equiv(&output_value));
}

#[test]
fn not_beta_equiv_by_var_both_bound() {
    // λx. λy. x x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("x"))),
                argument: Box::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. y x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("y"))),
                argument: Box::new(Value::Variable(String::from("x"))),
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
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("x"))),
                argument: Box::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. (λy. y) x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Application {
            function: Box::new(Value::Lambda {
                parameter: String::from("y"),
                body: Box::new(Value::Variable(String::from("y"))),
            }),
            argument: Box::new(Value::Variable(String::from("x"))),
        }),
    };

    assert!(!left.beta_equiv(&right));
}

#[test]
fn not_beta_equiv_by_var_both_free() {
    // λx. λy. a x
    let left = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("a"))),
                argument: Box::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. b x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("b"))),
                argument: Box::new(Value::Variable(String::from("x"))),
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
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("x"))),
                argument: Box::new(Value::Variable(String::from("x"))),
            }),
        }),
    };

    // λx. λy. b x
    let right = Value::Lambda {
        parameter: String::from("x"),
        body: Box::new(Value::Lambda {
            parameter: String::from("y"),
            body: Box::new(Value::Application {
                function: Box::new(Value::Variable(String::from("b"))),
                argument: Box::new(Value::Variable(String::from("x"))),
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
        body: Box::new(Value::Lambda {
            parameter: String::from("m"),
            body: Box::new(Value::Lambda {
                parameter: String::from("f"),
                body: Box::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: Box::new(Value::Application {
                        function: Box::new(Value::Application {
                            function: Box::new(Value::Variable(String::from(
                                "n",
                            ))),
                            argument: Box::new(Value::Variable(String::from(
                                "f",
                            ))),
                        }),
                        argument: Box::new(Value::Application {
                            function: Box::new(Value::Application {
                                function: Box::new(Value::Variable(
                                    String::from("m"),
                                )),
                                argument: Box::new(Value::Variable(
                                    String::from("f"),
                                )),
                            }),
                            argument: Box::new(Value::Variable(String::from(
                                "x",
                            ))),
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
        body: Box::new(Value::Lambda {
            parameter: String::from("m"),
            body: Box::new(Value::Lambda {
                parameter: String::from("f"),
                body: Box::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: Box::new(Value::Application {
                        function: Box::new(Value::Application {
                            function: Box::new(Value::Variable(String::from(
                                "n",
                            ))),
                            argument: Box::new(Value::Variable(String::from(
                                "f",
                            ))),
                        }),
                        argument: Box::new(Value::Application {
                            function: Box::new(Value::Application {
                                function: Box::new(Value::Variable(
                                    String::from("m"),
                                )),
                                argument: Box::new(Value::Variable(
                                    String::from("f"),
                                )),
                            }),
                            argument: Box::new(Value::Variable(String::from(
                                "x",
                            ))),
                        }),
                    }),
                }),
            }),
        }),
    };

    // λm. λn. λs. λz. m s (n s z)
    let right = Value::Lambda {
        parameter: String::from("m"),
        body: Box::new(Value::Lambda {
            parameter: String::from("n"),
            body: Box::new(Value::Lambda {
                parameter: String::from("s"),
                body: Box::new(Value::Lambda {
                    parameter: String::from("z"),
                    body: Box::new(Value::Application {
                        function: Box::new(Value::Application {
                            function: Box::new(Value::Variable(String::from(
                                "m",
                            ))),
                            argument: Box::new(Value::Variable(String::from(
                                "s",
                            ))),
                        }),
                        argument: Box::new(Value::Application {
                            function: Box::new(Value::Application {
                                function: Box::new(Value::Variable(
                                    String::from("n"),
                                )),
                                argument: Box::new(Value::Variable(
                                    String::from("s"),
                                )),
                            }),
                            argument: Box::new(Value::Variable(String::from(
                                "z",
                            ))),
                        }),
                    }),
                }),
            }),
        }),
    };

    assert!(left.beta_equiv(&right));
}
