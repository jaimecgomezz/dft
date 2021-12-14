use csv::Writer;
use std::error::Error;

use crate::definitions::traits::ToAdapter;
use crate::definitions::types::{Fields, OutputWriter, Records};

pub struct Adapter;

impl ToAdapter for Adapter {
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