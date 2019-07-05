use vm_runtime_tests::compile_and_execute;

fn main() {
    let program = String::from(
        "
        main() {
            return;
        }
        ",
    );
    match compile_and_execute(&program, vec![]) {
        Ok(_) => println!("Success"),
        _ => println!("Failure"),
    }
}
