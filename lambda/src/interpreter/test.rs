use super::run_once;
use super::Interpreter;
use crate::value::Value;

#[test]
fn capture() {
    //λa. (λx. λa. x a) a
    let input_value = Value::Lambda {
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

    assert_eq!(run_once(input_value), output_value);
}

#[test]
fn rename_capture() {
    // λa_. λa. (λx. λa. x a a_) a
    let input_value = Value::Lambda {
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

    assert_eq!(run_once(input_value), output_value);
}

#[test]
fn two_power_three() {
    // (λf. λx. f (f (f x))) (λf. λx. f (f x))
    let input_value = Value::Application {
        function: Box::new(Value::church_numeral(3)),
        argument: Box::new(Value::church_numeral(2)),
    };

    // λf. λx. f (f (f (f (f (f (f (f x)))))))
    let output_value = Value::church_numeral(8);
    assert!(run_once(input_value).beta_equiv(&output_value));
}

#[test]
fn steps() {
    // (λx. x x x) (λy. y) (λz. z)
    let input_value = Value::Application {
        function: Box::new(Value::Application {
            function: Box::new(Value::Lambda {
                parameter: String::from("x"),
                body: Box::new(Value::Application {
                    function: Box::new(Value::Application {
                        function: Box::new(Value::Variable(String::from("x"))),
                        argument: Box::new(Value::Variable(String::from("x"))),
                    }),
                    argument: Box::new(Value::Variable(String::from("x"))),
                }),
            }),
            argument: Box::new(Value::Lambda {
                parameter: String::from("y"),
                body: Box::new(Value::Variable(String::from("y"))),
            }),
        }),
        argument: Box::new(Value::Lambda {
            parameter: String::from("z"),
            body: Box::new(Value::Variable(String::from("z"))),
        }),
    };

    // (λz. z)
    let output_value = Value::Lambda {
        parameter: String::from("z"),
        body: Box::new(Value::Variable(String::from("z"))),
    };

    let mut interpreter = Interpreter::new(input_value);
    interpreter.run_all();
    assert!(interpreter.output().beta_equiv(&output_value));
    assert_eq!(interpreter.steps(), 4);
}
