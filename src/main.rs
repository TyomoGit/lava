use std::{process::{Command, Stdio}, env};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let java_file_index = args
        .iter()
        .rposition(|x| x.ends_with(".java"))
        .expect("🌀lava: no java file found");

    let java_file = &args[java_file_index];
    let options_and_compile_targets = &args[1..java_file_index];
    let arguments = &args[java_file_index+1..];

    let (options, compile_targets) = calc_compile_options_and_targets(options_and_compile_targets, java_file);

    compile(&options, &compile_targets);
    run(&options, java_file, arguments);
}

/// Returns a vector of all the files to be compiled
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `java_file` - The java file to be compiled
fn calc_compile_options_and_targets<'a>(options_and_ctargets: &'a[String], java_file: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {

    let mut options = Vec::<&str>::new();
    let mut compile_targets = Vec::<&str>::new();

    for element in options_and_ctargets {
        if element.ends_with(".java") {
            compile_targets.push(element);
        } else {
            options.push(element);
        }
    }

    if compile_targets.is_empty() {
        compile_targets.push(java_file);
    }

    (options, compile_targets)
}

/// Compiles the java files
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `compile_targets` - The files to be compiled
/// * `arguments` - The arguments passed to the java program
fn compile(options: &[&str], compile_targets: &[&str]) {
    let child_compile = Command::new("javac")
        .args(options)
        .args(compile_targets)
        // .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("🌀lava: failed to execute child");

    let output = child_compile.wait_with_output().expect("🌀lava: failed to wait on child(compilation)");
    if output.status.code().unwrap() != 0 {
        panic!("🌀lava: compilation failed");
    }
}

/// Runs the java program
/// 
/// # Arguments
/// 
/// * `options` - The options passed to lava
/// * `java_file` - The java file to be compiled
/// * `arguments` - The arguments passed to the java program
fn run(options: &[&str], java_file: &str, arguments: &[String]) {
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