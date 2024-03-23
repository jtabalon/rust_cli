use clap::Parser;

#[derive(Parser)]

struct Cli {
    // pattern to look for
    pattern: String,

    // path to the file to read
    path: std::path::PathBuf,
}


fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();

    let result = std::fs::read_to_string("test.txt");


    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    match result {
        Ok(content) => { println!("File content: {}", content); }
        Err(error) => { println!("oopsies: {}", error); }
    }


    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
