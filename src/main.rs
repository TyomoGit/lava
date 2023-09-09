use std::{process::{Command, Stdio}, env};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let java_file_index = args
        .iter()
        .rposition(|x| x.ends_with(".java"))
        .expect("🌀lava: no java file found");

    let java_file = &args[java_file_index];
    let options = args[1..java_file_index].to_vec();
    let arguments = &args[java_file_index+1..];

    let compile_files = compile_files(&options, java_file);

    let result = compile(&options, &compile_files, arguments);
    if result.is_err() { return; }
    run(&options, java_file, arguments);
    
}

fn compile_files(options: &[String], java_file: &str) -> Vec<String> {
    let mut options_r = options.to_vec();
    options_r.reverse();
    let mut compile_files = options_r
        .into_iter()
        .filter(|x| x.ends_with(".java"))
        .collect::<Vec<String>>();

    if compile_files.is_empty() {
        compile_files.push(java_file.to_owned());
    }

    compile_files
}

fn compile(options: &Vec<String>, compile_files: &[String], arguments: &[String]) -> Result<(), ()>{
    let child_compile = Command::new("javac")
        .args(options)
        .args(compile_files)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("🌀lava: failed to execute child");

    let output = child_compile.wait_with_output().expect("🌀lava: failed to wait on child(compilation)");
    if output.status.code().unwrap() != 0 {
        println!("🌀lava: compilation failed");
        return Err(());
    }

    Ok(())
}

fn run(options: &Vec<String>, java_file: &str, arguments: &[String]) {
    let child_run: std::process::Child = Command::new("java")
        .args(options)
        .arg(java_file.split('.').next().unwrap())
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("🌀lava: failed to execute child");

    let output = child_run.wait_with_output().expect("🌀lava: failed to wait on child(execution)");
    if output.status.code().unwrap() != 0 {
        println!("🌀lava: execution failed");
    }
}