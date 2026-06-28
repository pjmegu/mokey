use crate::ast;

pub fn generate(ast: ast::Root) -> String {
    let int = ast.int;

    format!(
        r"
int main(void) {{
    return {int};
}}
"
    )
}
