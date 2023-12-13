// fn main() {
//     println!("Hello, world!");
// }

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::Reader;

struct CsvTokenizer {
    file_path: String,
}

impl CsvTokenizer {
    fn new(file_path: &str) -> CsvTokenizer {
        CsvTokenizer {
            file_path: file_path.to_string(),
        }
    }

    fn tokenize(&self) -> Result<HashMap<usize, Vec<String>>, Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let mut reader = Reader::from_reader(file);
        let mut result = HashMap::new();
        for (i, record) in reader.records().enumerate() {
            let record = record?;
            result.insert(i, record.iter().map(|s| s.to_string()).collect());
        }
        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = CsvTokenizer::new("data/test_data.csv");
    let tokenized_data = tokenizer.tokenize()?;
    for (line, tokens) in tokenized_data {
        println!("Line {}: {:?}", line, tokens);
    }
    Ok(())
}
