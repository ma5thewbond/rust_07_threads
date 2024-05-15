use core::fmt::Formatter;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

///My implementation of csv parsing structure
pub struct MyCsv {
    separator: char,
    lines_count: u32,
    columns_count: usize,
    lines: Vec<Vec<String>>,
    sizes: Vec<usize>,
    initialized: bool,
    file_name: String,
}

impl MyCsv {
    /// Sort of constructor and initializer
    pub fn new(filename: String, sep: Option<char>) -> Result<Self, Box<dyn Error>> {
        let s = Self {
            separator: sep.unwrap_or(','),
            lines_count: 0,
            columns_count: 0,
            lines: Vec::new(),
            sizes: Vec::new(),
            initialized: false,
            file_name: filename,
        };

        return Ok(s);
    }

    ///main method for parsing csv after csv object is initialized with header data
    pub fn parse_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let mut first = true;
        let lines = MyCsv::read_from_file(&self.file_name)?;
        for fline in lines.flatten() {
            let line = self.parse_line(fline);
            match line {
                Err(msg) => {
                    eprintln!("Input processing ended with error: {msg}");
                    break;
                }
                Ok(line) => {
                    if first {
                        first = false;
                        self.columns_count = line.len();
                        self.initialized = true;
                    }
                    self.lines.push(line);
                }
            }
        }
        return Ok(());
    }

    /// parse line into Vec<String> separated by ,
    fn parse_line(&mut self, mut input: String) -> Result<Vec<String>, Box<dyn Error>> {
        if input == "\r\n" {
            return Err("End of input".into());
        }
        input = input.replace("\r\n", "");
        let line: Vec<String> = input.split(self.separator).map(|s| s.to_string()).collect();
        if self.initialized && line.len() != self.columns_count {
            return Err(format!(
                "Columnd count incorrect (was {}, should be {})",
                line.len(),
                self.columns_count
            )
            .into());
        }
        if !self.initialized {
            for h in line.iter() {
                self.sizes.push(h.len());
            }
        } else {
            for (i, v) in line.iter().enumerate() {
                if v.len() > self.sizes[i] {
                    self.sizes[i] = v.len();
                }
            }
            self.lines_count += 1;
        }
        return Ok(line);
    }

    fn read_from_file(filename: &String) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
        let p = Path::new(filename.trim());
        let file = File::open(p)?;
        return Ok(BufReader::new(file).lines());
    }
}

/// implement Display trait, so csv can be easily displayed in console
impl fmt::Display for MyCsv {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut str = format!(
            "Total lines: {}, Total columns: {}\n",
            &self.lines_count, &self.columns_count
        );
        let max_length: usize = self.sizes.iter().sum::<usize>() + (1 as usize); // Lukas, please, how to do this more inteligent way?
        str.push_str(&*format!("{:-<1$}\n", "-", self.columns_count + max_length));
        for line in self.lines.iter() {
            str.push('|');
            for (i, col) in line.iter().enumerate() {
                str.push_str(&*format!("{:01$}", col, self.sizes[i]));
                str.push('|');
            }
            str.push_str(&*format!(
                "\n{:-<1$}\n",
                "-",
                self.columns_count + max_length
            ));
        }
        fmt.write_str(&str)?;
        return Ok(());
    }
}
