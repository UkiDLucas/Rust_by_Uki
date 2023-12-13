// fn main() {
//     println!("Hello, world!");
// }

//use std::collections::Vec;
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

    fn tokenize(&self) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let mut reader = Reader::from_reader(file);
        let mut results: Vec<Vec<String>> = Vec::new();

        for (_, record) in reader.records().enumerate() {
            if let Ok(record) = record {
                let fields: Vec<String> = record.iter().map(|field| field.to_string()).collect();
                results.push(fields);
            }
        }

        Ok(results)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = CsvTokenizer::new("data/test_data.csv");
    let tokenized_data = tokenizer.tokenize()?;
    for tokens in &tokenized_data {
        println!("tokens: {:?}", tokens);
    }
    Ok(())
}
