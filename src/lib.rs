mod context;
mod ast;
mod codegen;
mod extcc;
mod hir;
mod ident;
mod lexer;
mod parser;

#[cfg(test)]
mod test {
    use std::{path::Path, process::Command};

    use indoc::indoc;

    use super::*;

    fn compile_and_run(src: &str) {
        let ctx = context::Ctx::new();
        let (tokens, error) = lexer::tokenize(src);
        assert_eq!(error, vec![]);
        println!("{:?}", tokens);
        let program = parser::parse(src, &tokens, &ctx).unwrap();
        println!("{:?}", program);
        let c_filename = Path::new("tmp.c");
        let mut c_file = std::fs::File::create(c_filename).unwrap();
        let mut cg = codegen::Codegen::new(&program, &mut c_file);
        cg.generate().unwrap();
        let out_filename = Path::new("./tmp");
        extcc::compile(&c_filename, &out_filename).unwrap();
        let out = Command::new(out_filename).output().unwrap();
        println!("{out:?}");
    }

    #[test]
    fn test() {
        compile_and_run("1");
        compile_and_run(indoc! {"
            func hello():
                var x = 2147483647
        "});
    }
}
