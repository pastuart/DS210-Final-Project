use std::error::Error;
use std::fmt;
use std::process;
use std::io;
use csv;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug, Clone)]
pub struct DataFrame {
    columns: Vec<Vec<ColumnVal>>,
    rows: usize,
    col_labels: Vec<String>,
}

// For returning errors
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

impl DataFrame {
    pub fn new(n: usize, cols: usize) -> Self {
        Self {
            columns: vec![vec![]],
            rows: n,
            col_labels: vec![],
        }
    }

    pub fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        //println!("Hello Boston!");
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path).expect("Couldn't find CSV file");
        let mut first_row = true;
        for result in rdr.records() {
            //println!("Hello Mass!");
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let r = result.unwrap();
            let mut row: Vec<ColumnVal> = vec![];
            if first_row {
                for elem in r.iter() { 
                    //println!("Hello World!");
                    self.col_labels.push(elem.to_string()); //orders the column labels
                }
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                println!("{:?}", elem);
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<bool>().unwrap())),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    4 => row.push(ColumnVal::Four(elem.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            self.columns.push(row);
            // Put the data into the dataframe
        }
        Ok(())
    }

    pub fn print(&self) {
        // print the labels
        for label in &self.col_labels {
            print!("{:<2?}    ", label);
        }
        println!();
        let mut i = 0;
        for row in &self.columns {
            for val in &self.columns[i] {
                match val {
                    ColumnVal::One(a) => print!("{:<2?}    ", a),
                    ColumnVal::Two(b) => print!("{:<2?}    ", b),
                    ColumnVal::Three(c) => print!("{:<2?}    ", c),
                    ColumnVal::Four(d) =>  print!("{:<2?}    ", d),
                }
            }
            println!();
            i += 1;
        }
        println!()
        // print the data
    }
}