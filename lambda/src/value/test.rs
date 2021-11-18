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
