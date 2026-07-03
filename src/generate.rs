use crate::ast;

pub fn generate(ast: ast::Root) -> String {
    let int = match ast.expr {
        ast::Expr::Int(i) => i,
        _ => todo!()
    };

    format!(
        r"
int main(void) {{
    return {int};
}}
"
    )
}
