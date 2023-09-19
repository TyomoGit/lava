use std::{process::{Command, Stdio}, env, io::{Result, Error, ErrorKind}};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let java_file_index = match args
        .iter()
        .rposition(|x| x.ends_with(".java")) {
            Some(index) => index,
            None => { println!("🌀lava: no java file found"); return;}
        };
        

    let java_file = &args[java_file_index];
    let options_and_compile_targets = &args[1..java_file_index];
    let arguments = &args[java_file_index+1..];

    let (options, compile_targets) = calc_compile_options_and_targets(options_and_compile_targets, java_file);

    if let Err(e) = compile(&options, &compile_targets) {
        println!("{}", e);
        return;
    }

    if let Err(e) = run(&options, java_file, arguments) {
        println!("{}", e);
    }
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
        compile_targets = vec![java_file];
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
fn compile(options: &[&str], compile_targets: &[&str]) -> Result<()>{
    let output = Command::new("javac")
        .args(options)
        .args(compile_targets)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    if output.status.code().unwrap() != 0 {
        return Err(Error::new(ErrorKind::Other, "🌀lava: compilation failed"));
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
fn run(options: &[&str], java_file: &str, arguments: &[String]) -> Result<()>{
    let child_run: std::process::Child = Command::new("java")
        .args(options)
        .arg(java_file.split('.').next().expect("🌀lava: no java file found"))
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;

    let output = child_run.wait_with_output()?;
    if output.status.code().expect("🌀lava: failed to get exit code") != 0 {
        return Err(Error::new(ErrorKind::Other, "🌀lava: execution failed"));
    }

    Ok(())
}