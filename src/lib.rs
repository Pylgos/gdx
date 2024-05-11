
mod ast;
mod lexer;
mod parser;
mod ident;
mod codegen;
mod extcc;

#[cfg(test)]
mod test {
    use std::{path::Path, process::Command};

    use super::*;

    fn compile_and_run(src: &str) {
        let (tokens, error) = lexer::tokenize(src);
        assert_eq!(error, vec![]);
        let ident_cache = ident::IdentCache::new();
        let program = parser::parse(src, &tokens, &ident_cache).unwrap();
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
    }
}