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
fn two_power_three() {
    // (λf. λx. f (f (f x))) (λf. λx. f (f x))
    let mut input_value = Value::Application {
        function: Box::new(Value::church_numeral(3)),
        argument: Box::new(Value::church_numeral(2)),
    };

    // λf. λx. f (f (f (f (f (f (f (f x)))))))
    let output_value = Value::church_numeral(8);

    input_value.reduce();
    assert_eq!(input_value, output_value);
}
