use super::run_once;
use super::Interpreter;
use crate::value::NestedValue;
use crate::value::Value;

#[test]
fn capture() {
    //λa. (λx. λa. x a) a
    let input_value = Value::Lambda {
        parameter: String::from("a"),
        body: NestedValue::new(Value::Application {
            function: NestedValue::new(Value::Lambda {
                parameter: String::from("x"),
                body: NestedValue::new(Value::Lambda {
                    parameter: String::from("a"),
                    body: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Variable(
                            String::from("x"),
                        )),
                        argument: NestedValue::new(Value::Variable(
                            String::from("a"),
                        )),
                    }),
                }),
            }),
            argument: NestedValue::new(Value::Variable(String::from("a"))),
        }),
    };

    //λa. (λa_. a a_)
    let output_value = Value::Lambda {
        parameter: String::from("a"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("a_"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("a"))),
                argument: NestedValue::new(Value::Variable(String::from("a_"))),
            }),
        }),
    };

    assert_eq!(run_once(input_value), output_value);
}

#[test]
fn rename_capture() {
    // λa_. λa. (λx. λa. x a a_) a
    let input_value = Value::Lambda {
        parameter: String::from("a_"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("a"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Lambda {
                    parameter: String::from("x"),
                    body: NestedValue::new(Value::Lambda {
                        parameter: String::from("a"),
                        body: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Application {
                                function: NestedValue::new(Value::Variable(
                                    String::from("x"),
                                )),
                                argument: NestedValue::new(Value::Variable(
                                    String::from("a"),
                                )),
                            }),
                            argument: NestedValue::new(Value::Variable(
                                String::from("a_"),
                            )),
                        }),
                    }),
                }),
                argument: NestedValue::new(Value::Variable(String::from("a"))),
            }),
        }),
    };

    // λa_. λa. (λa__. a a__ a_)
    let output_value = Value::Lambda {
        parameter: String::from("a_"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("a"),
            body: NestedValue::new(Value::Lambda {
                parameter: String::from("a__"),
                body: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Variable(
                            String::from("a"),
                        )),
                        argument: NestedValue::new(Value::Variable(
                            String::from("a__"),
                        )),
                    }),
                    argument: NestedValue::new(Value::Variable(String::from(
                        "a_",
                    ))),
                }),
            }),
        }),
    };

    assert_eq!(run_once(input_value), output_value);
}

#[test]
fn two_power_three() {
    // (λf. λx. f (f (f x))) (λf. λx. f (f x))
    let input_value = Value::Application {
        function: NestedValue::new(Value::church_numeral(3)),
        argument: NestedValue::new(Value::church_numeral(2)),
    };

    // λf. λx. f (f (f (f (f (f (f (f x)))))))
    let output_value = Value::church_numeral(8);
    assert!(run_once(input_value).beta_equiv(&output_value));
}

#[test]
fn steps() {
    // (λx. x x x) (λy. y) (λz. z)
    let input_value = Value::Application {
        function: NestedValue::new(Value::Application {
            function: NestedValue::new(Value::Lambda {
                parameter: String::from("x"),
                body: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Variable(
                            String::from("x"),
                        )),
                        argument: NestedValue::new(Value::Variable(
                            String::from("x"),
                        )),
                    }),
                    argument: NestedValue::new(Value::Variable(String::from(
                        "x",
                    ))),
                }),
            }),
            argument: NestedValue::new(Value::Lambda {
                parameter: String::from("y"),
                body: NestedValue::new(Value::Variable(String::from("y"))),
            }),
        }),
        argument: NestedValue::new(Value::Lambda {
            parameter: String::from("z"),
            body: NestedValue::new(Value::Variable(String::from("z"))),
        }),
    };

    // (λz. z)
    let output_value = Value::Lambda {
        parameter: String::from("z"),
        body: NestedValue::new(Value::Variable(String::from("z"))),
    };

    let mut interpreter = Interpreter::new(input_value);
    interpreter.run_all();
    assert!(interpreter.output().beta_equiv(&output_value));
    assert_eq!(interpreter.steps(), 4);
}
