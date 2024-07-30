use std::{io::Write, process::Stdio};

fn main() -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();

    print!("[1] Parsing Arguments");
    stdout.flush()?;

    let mut args = std::env::args().collect::<Vec<String>>();

    let mut lines: Vec<String> = Vec::new();

    if args.contains(&"--help".to_string()) {
        println!("\rbrainfork [file]               ");
        std::process::exit(0);
    }

    args.remove(0);

    if args.len() != 1 {
        println!("\rWrong amount of arguments!     ");
        std::process::exit(1);
    }

    print!("\r[2] Generating Rust");
    stdout.flush()?;

    let include = include_str!("../include.rs").to_string();

    lines.append(&mut include.lines().map(|x| x.to_string()).collect());

    // parse file

    let file_name = args[0].clone();

    if !std::path::PathBuf::from(&file_name).exists() {
        println!("\rSource file doesn't exist!       ");
        std::process::exit(1);
    }

    let file = std::fs::read_to_string(&file_name)?;

    let mut loop_stack = Vec::new();
    let mut loop_count = 0;

    for char in file.chars() {
        match char {
            '>' => {
                lines.push("inc_one(&mut ptr, &stack);".to_string());
            }
            '<' => {
                lines.push("dec_one(&mut ptr, &stack);".to_string());
            }
            '+' => {
                lines.push("inc_val(&mut ptr, &mut stack);".to_string());
            }
            '-' => {
                lines.push("dec_val(&mut ptr, &mut stack);".to_string());
            }
            '.' => {
                lines.push("print_val(&mut ptr, &stack);".to_string());
            }
            ',' => {
                lines.push("read_val(&mut ptr, &mut stack);".to_string());
            }
            '[' => {
                loop_count += 1;
                loop_stack.push(loop_count);
                lines.push(format!(
                    "while stack[ptr as usize] != 0 {{ // Loop start {}",
                    loop_count
                ));
            }
            ']' => {
                if let Some(loop_num) = loop_stack.pop() {
                    lines.push(format!("}} // Loop end {}", loop_num));
                } else {
                    println!("\rUnmatched closing bracket!    ");
                    std::process::exit(1);
                }
            }
            _ => (), // ignore the char
        }
    }

    lines.push(String::from("close();"));
    lines.push(String::from("}"));

    // write the file

    print!("\r                           ");
    print!("\r[03] Writing to file");
    stdout.flush()?;

    let file_data = file_name.split('.').next().unwrap_or("main").to_string();

    std::fs::write(format!("{}.rs", file_data), lines.join("\n"))?;

    print!("\r                           ");
    print!("\r[04] Compiling Rust");
    stdout.flush()?;

    std::process::Command::new("rustc")
        .arg(format!("{}.rs", file_data))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| {
            println!("\rError while compiling: {:?}", e);
            std::process::exit(1);
        })
        .wait()
        .unwrap_or_else(|e| {
            println!("\rError while compiling: {:?}", e);
            std::process::exit(1);
        });

    print!("\r                                      "); // clear the Stdout
    stdout.flush()?;
    println!("\r[05] Compiled");

    Ok(())
}
