use crate::compiler::{
    expansor::expand, lexer::generate_tokens, parser::parse,
};
use crate::interpreter::run_once;
use crate::value::{NestedValue, Value};

use pinguim_language::error::Diagnostics;
use std::fs::File;
use std::io::Write;

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

#[test]
fn expand_bindings_with_same_name() {
    let code = "let\na = \\n. \\f x. n f (f x);\na = \\m n. m a n;\nmul = \\m n. m (a n) 0;\nin\nmul 3 5";
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

    let first_a = Value::Lambda {
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

    let second_a = Value::Lambda {
        parameter: String::from("m"),
        body: NestedValue::new(Value::Lambda {
            parameter: String::from("n"),
            body: NestedValue::new(Value::Application {
                function: NestedValue::new(Value::Application {
                    function: NestedValue::new(Value::Variable(String::from(
                        "m",
                    ))),
                    argument: NestedValue::new(first_a),
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
                        function: NestedValue::new(second_a),
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

#[test]
fn run_expanded_value() {
    let code = "let\nsucc = \\n. \\f x. n f (f x);\nadd = \\m n. m succ n;\nmul = \\m n. m (add n) 0;\nin\nmul 3 5";
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics).unwrap();
    let value = expand(&ast, &mut diagnostics);
    let expected_result = 15;
    let result = run_once(value.unwrap()).church_numeral_to_int();

    let mut f_generated = File::create("generated.txt").unwrap();
    let mut f_expected = File::create("expected.txt").unwrap();

    write!(&mut f_generated, "{:#?}\n", result).unwrap();
    write!(&mut f_expected, "{:#?}\n", expected_result).unwrap();

    //assert_eq!(result, expected_result);
}
