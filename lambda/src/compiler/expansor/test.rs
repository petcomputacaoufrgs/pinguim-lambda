use crate::compiler::expansor::expand;
use crate::compiler::lexer::generate_tokens;
use crate::compiler::parser::{ast, parse};
use crate::value::{NestedValue, Value};

use pinguim_language::error::Diagnostics;

#[test]
fn expand_code() {
    let code = "let\nsucc = \\n. \\f x. n f (f x);\nadd = \\m n. m succ n;\nmul = \\m n. m (add n) 0;\nin\nmul 3 5";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics).unwrap();
    let value = expand(&ast, &mut diagnostics);

    let church_zero = Value::Lambda {
        parameter: String::from("f"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("x"),
            body: NestedValue::new(Value::Variable(String::from("x"))),
        }),
    };

    let church_three = Value::Lambda {
        parameter: String::from("f"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("x"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("f"))),
                argument: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Variable(String::from(
                        "f",
                    ))),
                    argument: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Variable(
                            String::from("f"),
                        )),
                        argument: NestedValue::new(Value::Variable(
                            String::from("x"),
                        )),
                    }),
                }),
            }),
        }),
    };

    let church_five = Value::Lambda {
        parameter: String::from("f"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("x"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Variable(String::from("f"))),
                argument: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Variable(String::from(
                        "f",
                    ))),
                    argument: NestedValue::new(Value::Application {
                        function: NestedValue::new(Value::Variable(
                            String::from("f"),
                        )),
                        argument: NestedValue::new(Value::Application {
                            function: NestedValue::new(Value::Variable(
                                String::from("f"),
                            )),
                            argument: NestedValue::new(Value::Application {
                                function: NestedValue::new(Value::Variable(
                                    String::from("f"),
                                )),
                                argument: NestedValue::new(Value::Variable(
                                    String::from("x"),
                                )),
                            }),
                        }),
                    }),
                }),
            }),
        }),
    };

    let succ_value = Value::Lambda {
        parameter: String::from("n"),
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
                        function: NestedValue::new(Value::Variable(
                            String::from("f"),
                        )),
                        argument: NestedValue::new(Value::Variable(
                            String::from("x"),
                        )),
                    }),
                }),
            }),
        }),
    };

    let add_value = Value::Lambda {
        parameter: String::from("m"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("n"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Variable(String::from(
                        "m",
                    ))),
                    argument: NestedValue::new(succ_value),
                }),
                argument: NestedValue::new(Value::Variable(String::from("n"))),
            }),
        }),
    };

    let mul_value = Value::Lambda {
        parameter: String::from("m"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("n"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Variable(String::from(
                        "m",
                    ))),
                    argument: NestedValue::new(Value::Application {
                        function: NestedValue::new(add_value),
                        argument: NestedValue::new(Value::Variable(
                            String::from("n"),
                        )),
                    }),
                }),
                argument: NestedValue::new(church_zero),
            }),
        }),
    };

    let expected_value = Value::Application {
        function: NestedValue::new(Value::Application {
            function: NestedValue::new(mul_value),
            argument: NestedValue::new(church_three),
        }),
        argument: NestedValue::new(church_five),
    };

    assert!(diagnostics.is_ok());
    assert_eq!(value, Some(expected_value));
}
