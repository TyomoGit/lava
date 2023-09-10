use std::{process::{Command, Stdio}, env};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let java_file_index = args
        .iter()
        .rposition(|x| x.ends_with(".java"))
        .expect("🌀lava: no java file found");

    let java_file = &args[java_file_index];
    let options = &args[1..java_file_index];
    let arguments = &args[java_file_index+1..];

    let compile_targets = compile_targets(options, java_file);

    let result = compile(options, &compile_targets, arguments);
    if result.is_err() { return; }

    run(options, java_file, arguments);
}

/// Returns a vector of all the files to be compiled
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `java_file` - The java file to be compiled
fn compile_targets(options: &[String], java_file: &str) -> Vec<String> {
    let mut options_r = options.to_vec();
    options_r.reverse();
    let mut compile_targets = options_r
        .into_iter()
        .filter(|x| x.ends_with(".java"))
        .collect::<Vec<String>>();

    if compile_targets.is_empty() {
        compile_targets.push(java_file.to_owned());
    }

    compile_targets
}

/// Compiles the java files
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `compile_targets` - The files to be compiled
/// * `arguments` - The arguments passed to the java program
fn compile(options: &[String], compile_targets: &[String], arguments: &[String]) -> Result<(), ()>{
    let child_compile = Command::new("javac")
        .args(options)
        .args(compile_targets)
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

/// Runs the java program
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `java_file` - The java file to be compiled
/// * `arguments` - The arguments passed to the java program
fn run(options: &[String], java_file: &str, arguments: &[String]) {
    let child_run: std::process::Child = Command::new("java")
        .args(options)
        .arg(java_file.split('.').next().expect("🌀lava: no java file found"))
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("🌀lava: failed to execute child");

    let output = child_run.wait_with_output().expect("🌀lava: failed to wait on child(execution)");
    if output.status.code().expect("🌀lava: failed to get exit code") != 0 {
        println!("🌀lava: execution failed");
    }
}