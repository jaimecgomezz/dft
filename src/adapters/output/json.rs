use json::JsonValue;
use std::error::Error;

use crate::definitions::enums::Type;
use crate::definitions::traits::OutputAdapter;
use crate::definitions::types::{Fields, OutputWriter, Records};

pub struct Adapter;

impl OutputAdapter for Adapter {
    fn write(
        &self,
        mut writer: OutputWriter,
        fields: &Fields,
        records: &Records,
    ) -> Result<usize, Box<dyn Error>> {
        let mut writables = JsonValue::new_array();

        for record in records {
            if record.discarded {
                continue;
            }

            let mut writable = JsonValue::new_object();

            for (field, value) in fields.iter().zip(record.values.iter()) {
                let value = match field.typed {
                    Type::STRING => JsonValue::from(value.to_string()),
                    Type::NUMBER => match value.parse::<f64>() {
                        Ok(parsed) => JsonValue::from(parsed),
                        Err(_) => panic!(
                            "Invalid number value for {} in record {}",
                            field.name, record.id
                        ),
                    },
                    Type::BOOLEAN => match value.as_str() {
                        "true" | "t" | "1" => JsonValue::from(true),
                        "false" | "f" | "0" => JsonValue::from(false),
                        _ => panic!(
                            "Invalid bool value for {} in record {}",
                            field.name, record.id
                        ),
                    },
                };

                writable.insert(field.name.as_str(), value)?;
            }

            writables.push(writable)?;
        }

        writables.write_pretty(&mut writer, 4)?;

        Ok(writables.len())
    }
}
