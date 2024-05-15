use crate::mycsv::MyCsv;
use slug::slugify;
use std::env;
use std::error::Error;
use std::io;

pub mod mycsv;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1].as_str() == "?" {
        println!(
            "Missing required operation parameter. Possible parameters are:\nlowercase\nuppercase\nno-spaces\nslugify\nreverse\ntrim\ncsv"
        );
        return;
    } else {
        eprintln!("Selected operation: {}", args[1]);
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
