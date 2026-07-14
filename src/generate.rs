use crate::ir;

pub fn generate(ir: ir::IR) -> Result<String, GenCError> {
    Generator::new(ir).generate()
}

#[derive(Debug)]
pub enum GenCError {}

struct Generator {
    ops: Vec<ir::Op>,

    lines: Vec<String>,
}

impl Generator {
    fn new(ir: ir::IR) -> Self {
        Self {
            ops: ir.ops,
            lines: Vec::new(),
        }
    }

    fn generate(mut self) -> Result<String, GenCError> {
        for (vnum, op) in self.ops.iter_mut().enumerate() {
            let ty = get_ty_name(&op.result_type);
            match op.kind {
                ir::OpKind::NOP => {
                    continue;
                }
                ir::OpKind::ConstInt(i) => {
                    let expr = i.to_string();
                    self.lines.push(format!("{ty} v{vnum} = {expr};"));
                }
                ir::OpKind::Plus(lhs, rhs) => {
                    let expr = format!("v{lhs} + v{rhs}");
                    self.lines.push(format!("{ty} v{vnum} = {expr};"));
                }
            }
        }

        self.lines.push(format!("return v{};", self.ops.len() - 1));

        let c = self.lines.join("\n");
        Ok(c)
    }
}

fn get_ty_name(ty: &ir::VType) -> &str {
    match ty {
        ir::VType::Int => "int",
    }
}
