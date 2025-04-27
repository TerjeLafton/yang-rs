fn main() {
    let path = "module.yang";
    let parsed_module = yang_parser::parse(path).unwrap();
    yang_codegen::generate(parsed_module);
}
