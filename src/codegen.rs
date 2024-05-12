use crate::ast::{Expr, Lit, Program, Stmt, VarDef};



pub struct Codegen<'a, Dst: std::io::Write> {
    ast: &'a Program<'a>,
    dst: &'a mut Dst,
}

type Result<T> = std::result::Result<T, std::io::Error>;

impl<'a, Dst: std::io::Write> Codegen<'a, Dst> {
    pub fn new(ast: &'a Program<'a>, dst: &'a mut Dst) -> Self {
        Self {
            ast,
            dst,
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        self.gen_program(self.ast)?;
        Ok(())
    }

    fn gen_program(&mut self, prog: &Program) -> Result<()> {
        writeln!(self.dst, "int main() {{")?;
        for stmt in prog.stmt_list.stmts.iter() {
            self.gen_stmt(stmt)?;
        }
        writeln!(self.dst, "}}")?;
        Ok(())
    }

    fn gen_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        // match stmt {
        //     Stmt::Expr(expr) => {
        //         self.gen_expr(expr)?;
        //     }
        //     Stmt::VarDef(var_decl) => {

        //     }
        //     Stmt::FuncDef(func_def) => {

        //     }
        //     Stmt::Pass => {

        //     }
        // }
        // writeln!(self.dst, ";")?;
        Ok(())
    }

    fn gen_expr(&mut self, expr: &Expr) -> Result<()> {
        // match expr {
        //     Expr::Lit(lit) => {
        //         self.gen_lit(lit)?;
        //     }
        //     _ => unreachable!(),
        // }
        Ok(())
    }

    fn gen_lit(&mut self, lit: &Lit) -> Result<()> {
        match lit {
            Lit::Int(val) => {
                write!(self.dst, "{:}", val)?;
            }
        }
        Ok(())
    }

    fn gen_var_decl(&mut self, var_decl: &VarDef) -> Result<()> {
        // write!(self.dst, "")
        Ok(())
    }
}
