use crate::mycsv::MyCsv;
use slug::slugify;
use std::env;
use std::error::Error;
use std::io;
use std::thread;

pub mod mycsv;

//static mut WAIT_FOR_INPUT: RwLock<bool> = RwLock::new(false);

fn read_input(
    tread: flume::Sender<String>,
    rexec: flume::Receiver<bool>,
) -> Result<(), Box<dyn Error + Send>> {
    loop {
        let mut input = String::new();
        println!("Please enter next command and param in form <command> <param>[ENTER] or empty line to exit:");
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("User input was incorrect");
            continue;
        }
        input = String::from(input.trim());
        if input == "" {
            input = String::from("stop");
            tread.send(input).unwrap();
            return Ok(());
        }
        tread.send(input).unwrap();
        let _ = rexec.recv().unwrap(); // wait for go from execution thread to request new input
    }
}

fn execute_command(
    rread: flume::Receiver<String>,
    texec: flume::Sender<bool>,
) -> Result<(), Box<dyn Error + Send>> {
    loop {
        let msg = rread.recv().unwrap(); // wait for the command from the input thread
        if msg == "stop" {
            return Ok(());
        }

        let data = msg.split_once(' ');

        match data {
            Some((command, input)) => {
                let result = match command {
                    "lowercase" => lowercase(String::from(input)),
                    "uppercase" => uppercase(String::from(input)),
                    "no-spaces" => nospaces(String::from(input)),
                    "slugify" => slugify_input(String::from(input)),
                    "reverse" => reverse(String::from(input)),
                    "trim" => trim(String::from(input)),
                    "csv" => {
                        let csv_input = input.split_once(' ');
                        let csv = match csv_input {
                            Some((file_name, separator)) => {
                                if separator.len() == 0 {
                                    parse_csv(String::from(file_name), None)
                                } else {
                                    parse_csv(
                                        String::from(file_name),
                                        Some(separator.chars().nth(0).unwrap()),
                                    )
                                }
                            }
                            None => parse_csv(String::from(input), None),
                        };

                        match csv {
                            Ok(csv) => Ok(csv.to_string()),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err(format!("Unsupported operation {}, bye", command).into()),
                };

                match result {
                    Err(e) => {
                        eprintln!("Error '{}' happened in operation {}, bye", e, command);
                    }
                    Ok(str) => {
                        println!("Operation result: \n{}", str);
                    }
                }
            }
            None => {
                println!("Not enough data for command {}", msg);
            }
        }

        texec.send(true).unwrap(); // send input thread Go for new command input
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1].as_str() == "?" {
        println!(
            "Missing required operation parameter. Possible parameters are:\nlowercase\nuppercase\nno-spaces\nslugify\nreverse\ntrim\ncsv"
        );
        return;
    } else if args.len() == 1 {
        println!("Interactive mode");
        let (tread, rread) = flume::unbounded::<String>();
        let (texec, rexec) = flume::unbounded::<bool>();
        let rhandle = thread::spawn(|| read_input(tread, rexec));
        let ehandle = thread::spawn(|| execute_command(rread, texec));
        _ = rhandle.join().unwrap();
        _ = ehandle.join().unwrap();
        return;
    } else if args.len() >= 2 {
        println!("Selected operation: {}", args[1]);
    }

    let mut input = String::new();
    let result: Result<String, Box<dyn Error>>;

    println!("Enter text to modify: ");
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("User input was incorrect, bye");
        return;
    }

    result = match args[1].as_str() {
        "lowercase" => lowercase(input),
        "uppercase" => uppercase(input),
        "no-spaces" => nospaces(input),
        "slugify" => slugify_input(input),
        "reverse" => reverse(input),
        "trim" => trim(input),
        "csv" => {
            let mut separator = None;
            if args.len() > 2 {
                separator = Some(args[2].chars().nth(0).unwrap());
            }
            let csv = parse_csv(input, separator);
            match csv {
                Ok(csv) => Ok(csv.to_string()),
                Err(e) => Err(e),
            }
        }
        _ => {
            println!("Unsupported operation {}, bye", args[1]);
            return;
        }
    };

    match result {
        Err(e) => {
            eprintln!("Error '{}' happened in operation {}, bye", e, args[1]);
        }
        Ok(str) => {
            println!("Operation result: \n{}", str);
        }
    }
}

fn lowercase(mut str: String) -> Result<String, Box<dyn Error>> {
    str = str.to_lowercase();
    return Ok(str);
}

fn uppercase(mut str: String) -> Result<String, Box<dyn Error>> {
    str = str.to_uppercase();
    return Ok(str);
}

fn nospaces(mut str: String) -> Result<String, Box<dyn Error>> {
    str = str.replace(" ", "");
    return Ok(str);
}

fn slugify_input(mut str: String) -> Result<String, Box<dyn Error>> {
    str = slugify(str);
    return Ok(str);
}

fn reverse(mut str: String) -> Result<String, Box<dyn Error>> {
    let mut res: String = String::new();
    for x in str.chars().rev() {
        if x == '\n' {
            continue;
        }
        res.push(x);
    }
    str = res;
    return Ok(str);
}

fn trim(mut str: String) -> Result<String, Box<dyn Error>> {
    str = String::from(str.trim());
    return Ok(str);
}

fn parse_csv(str: String, separator: Option<char>) -> Result<MyCsv, Box<dyn Error>> {
    let mut my_csv = MyCsv::new(str, separator)?;
    my_csv.parse_csv()?;
    return Ok(my_csv);
}
