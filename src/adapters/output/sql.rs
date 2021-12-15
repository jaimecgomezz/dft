use std::error::Error;

use crate::definitions::enums::Type;
use crate::definitions::structs::Optionals;
use crate::definitions::traits::OutputAdapter;
use crate::definitions::types::{Fields, OutputWriter, Records};

pub struct Adapter;

impl Adapter {
    fn parse_value(&self, typed: &Type, value: String) -> String {
        match typed {
            Type::STRING => format!(r#"'{}'"#, value),
            _ => value,
        }
    }

    fn build_statement(&self, tname: &String, fields: Vec<String>, values: Vec<String>) -> String {
        let fields = fields.join(", ");
        let values = values.join(", ");

        format!("INSERT INTO {} ({}) VALUES ({});", tname, fields, values)
    }
}

impl OutputAdapter for Adapter {
    fn write(
        &self,
        mut writer: OutputWriter,
        fields: &Fields,
        records: &Records,
        optionals: Optionals,
    ) -> Result<usize, Box<dyn Error>> {
        let mut written = 0;

        for record in records {
            if record.discarded {
                continue;
            }

            let mut formatted: Vec<String> = vec![];

            for (field, value) in fields.iter().zip(record.values.iter()) {
                formatted.push(self.parse_value(&field.typed, value.to_string()));
            }

            let fields: Vec<String> = fields.into_iter().map(|f| f.name.to_string()).collect();

            writeln!(
                writer,
                "{}",
                self.build_statement(&optionals.tname, fields, formatted)
            )?;

            written += 1;
        }

        Ok(written)
    }
}
