use csv::{Reader, Writer};
use std::error::Error;

use crate::definitions::traits::Adapter;
use crate::definitions::types::{
    Field, Fields, InputReader, OutputWriter, Record, Records, Values,
};

pub struct CsvAdapter;

impl Adapter for CsvAdapter {
    fn read(&self, reader: InputReader) -> Result<(Fields, Records), Box<dyn Error>> {
        let mut result: Records = vec![];

        let mut reader = Reader::from_reader(reader);

        let fields: Fields = reader
            .headers()?
            .clone()
            .iter()
            .map(|header| Field::new(header, "string"))
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

    fn write(
        &self,
        writer: OutputWriter,
        fields: &Fields,
        records: &Records,
    ) -> Result<usize, Box<dyn Error>> {
        let mut written = 0;
        let mut writer = Writer::from_writer(writer);

        let headers: Vec<String> = fields.iter().map(|field| field.name.to_owned()).collect();

        writer.write_record(headers.iter())?;

        for record in records {
            writer.write_record(record.values.iter())?;

            written += 1;
        }

        Ok(written)
    }
}
