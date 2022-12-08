use crate::ast::*;

fn typecheck_expression(expr: &Expression) -> bool {
    true
}

pub fn typecheck(ast: Vec<Statement>) -> bool {
    for x in ast {
        match x {
            Statement::Expression(e) => {
                let check = typecheck_expression(&e);
                if !check {
                    return false;
                }
            },
            Statement::Print(expr) => {
                let check = typecheck_expression(&expr);
                if !check {
                    return false;
                }
            },
        }
    }
    true
}
