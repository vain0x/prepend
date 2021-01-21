use std::{
    env, fs,
    io::{self, Read, Seek, SeekFrom, Write},
    process,
};

enum Input {
    String(String),
    StdIn,
}

fn main() {
    let mut args = env::args();
    args.next();

    let mut edit_file_opt = None;
    let mut inputs = vec![];

    let mut help = false;
    let mut version = false;
    let mut verbatim = false;
    let mut escape = false;
    let mut newline = true;

    while let Some(arg) = args.next() {
        if !verbatim && arg.starts_with("-") {
            match arg.as_str() {
                "--" => verbatim = true,
                "-" | "--stdin" => inputs.push(Input::StdIn),
                "-e" | "--escape" => escape = true,
                "-n" | "--no-newline" => newline = false,
                "-h" | "--help" | "-help" => help = true,
                "-V" | "--version" => version = true,
                _ => {
                    eprintln!("ERROR: Illegal option '{}'. HINT: Use '--' delimiter to pass string starting with '-' as non-option argument.", arg);
                    std::process::exit(1)
                }
            }
            continue;
        }

        if edit_file_opt.is_none() {
            edit_file_opt = Some(arg);
            continue;
        }

        inputs.push(Input::String(arg));
    }

    if help {
        print_help();
        return;
    }

    if version {
        print_version();
        return;
    }

    let target = match edit_file_opt {
        Some(it) => it,
        None => {
            eprintln!("ERROR: Specify the filepath to update in arguments.");
            process::exit(1)
        }
    };

    if inputs.is_empty() {
        inputs.push(Input::StdIn);
    }

    let mut edit_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&target)
        .expect("opening file");

    let last = {
        let file_size = match edit_file.metadata() {
            Ok(metadata) => metadata.len() as usize,
            Err(_) => 0,
        };
        let mut last = Vec::with_capacity(file_size);
        edit_file.read_to_end(&mut last).expect("reading contents");
        last
    };

    let mut contents = {
        let estimated_size = inputs
            .iter()
            .map(|input| match input {
                Input::String(s) => s.len(),
                Input::StdIn => 0x1000,
            })
            .sum::<usize>();
        Vec::with_capacity(estimated_size)
    };

    for input in inputs {
        match input {
            Input::String(s) => {
                if escape {
                    // FIXME: Do correctly and more efficiently. Support \xHH.
                    let s = s
                        .replace("\\r", "\r")
                        .replace("\\n", "\n")
                        .replace("\\t", "\t")
                        .replace("\\\\", "\\");
                    contents.extend_from_slice(s.as_bytes());
                } else {
                    contents.extend_from_slice(s.as_bytes());
                }
            }
            Input::StdIn => {
                let stdin = io::stdin();
                let mut stdin = stdin.lock();
                let mut buf = [0_u8; 0x1000];

                loop {
                    let n = stdin.read(&mut buf).expect("reading from standard input");
                    if n == 0 {
                        break;
                    }

                    contents.extend_from_slice(&buf[..n]);
                }
            }
        }

        if newline {
            contents.push(b'\n');
        }
    }

    if !contents.is_empty() {
        edit_file.set_len(0).expect("truncating file");
        edit_file
            .seek(SeekFrom::Start(0))
            .expect("seeking to the beginning");
        edit_file.write_all(&contents).expect("writing to file");
        edit_file.write_all(&last).expect("writing to file");
    }
}

fn print_help() {
    print!(
        include_str!("./help.txt"),
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION")
    );
}

fn print_version() {
    println!(
        "{name} v{version}",
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION")
    );
}
