use csv::Reader;
use std::error::Error;

use crate::definitions::traits::InputAdapter;
use crate::definitions::types::{Field, Fields, InputReader, Record, Records, Values};

pub struct Adapter;

impl InputAdapter for Adapter {
    fn read(&self, reader: InputReader) -> Result<(Fields, Records), Box<dyn Error>> {
        let mut result: Records = vec![];

        let mut reader = Reader::from_reader(reader);

        let fields: Fields = reader
            .headers()?
            .clone()
            .iter()
            .map(|header| Field::new(header))
            .collect();

        let mut id: u64 = 0;
        for record in reader.into_records() {
            // TODO: prevent recollection if possible
            // This had to be done since if ommited it would carry the StringRecord type
            // out of the CsvAdapter context
            let values: Values = record?.iter().map(|e| e.to_string()).collect();

            result.push(Record::new(id, values));

            id += 1;
        }

        Ok((fields, result))
    }
}
