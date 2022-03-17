use crate::compiler::parser::ast;
use crate::value::{NestedValue, Value};
use pinguim_language::error::{Diagnostics, Error};

pub fn expand(
    ast: &ast::Program,
    diagnostics: &mut Diagnostics,
) -> Option<Value> {
    Expansor::new(ast).expand_program(diagnostics)
}

pub struct Expansor<'ast> {
    ast: &'ast ast::Program,
    expression: Value,
}

impl<'ast> Expansor<'ast> {
    fn new(ast: &'ast ast::Program) -> Self {
        Expansor { ast, expression: Value::dummy() }
    }

    fn expr_to_value(
        &self,
        expr: ast::Expr,
        diagnostics: &mut Diagnostics,
    ) -> Value {
        match expr {
            ast::Expr::Variable(var) => return Value::Variable(var.content),
            ast::Expr::Number(num) => return Value::church_numeral(num),
            ast::Expr::Application { function, argument } => {
                return Value::Application {
                    function: NestedValue::new(
                        self.expr_to_value(*function, diagnostics),
                    ),
                    argument: NestedValue::new(
                        self.expr_to_value(*argument, diagnostics),
                    ),
                }
            }
            ast::Expr::Lambda { parameter, body } => {
                return Value::Lambda {
                    parameter: parameter.content,
                    body: NestedValue::new(
                        self.expr_to_value(*body, diagnostics),
                    ),
                }
            }
        }
    }

    fn expand_program(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Option<Value> {
        let mut main = self.ast.main_expression.clone();
        let mut bindings = self.ast.bindings.clone();

        self.expression = self.expr_to_value(main, diagnostics);

        for binding in bindings {
            let mut binding_value =
                self.expr_to_value(binding.expression, diagnostics);
            self.expression.replace(&binding.name.content, &binding_value);
        }

        Some(self.expression.clone())
    }
}
