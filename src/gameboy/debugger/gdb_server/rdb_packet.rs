use atoi::FromRadix16;
use std::fmt::Error;

pub enum ParseError {
    MalformedPacket,
    IncorrectArgumentCount,
    UnknownCommand,
    IncorrectChecksum,
}

pub struct RdbPacket {
    pub data: Vec<String>,
}

impl RdbPacket {
    fn parse_data_arguments(data_raw: &str) -> Result<Vec<&str>, ParseError> {
        let mut fields: Vec<&str> = Vec::new();

        // Special cases
        if data_raw.starts_with("qL") {
            fields.push("qL");
            // TODO: Parse the qL fields, unused for now anwyway
        } else if data_raw.starts_with("Hc") {
            fields.push("Hc");
            // TODO: Parse the Hc fields, unused for now anwyway
        } else if data_raw.starts_with("p") {
            //let register_number_raw = data_raw.strip_prefix("p");
            //let register_number = atoi::reg

            fields.push("p");
        } else {
            fields = data_raw.split(&[';', ':', ',']).collect();
            if fields.len() < 1 {
                return Err(ParseError::IncorrectArgumentCount);
            }
        }
        Ok(fields)
    }

    pub fn packetify(response: &str) -> String {
        let checksum: usize = response.chars().map(|x| x as usize).sum::<usize>() & 0xFF;

        // TODO: Add escape for special characters + data packing
        format!("${}#{:02X}", response, checksum)
    }

    pub fn parse(_packet: &str) -> Result<Self, ParseError> {
        let mut packet = _packet.to_string();

        println!("Raw packet: {}, length: {}", packet, packet.len());
        if !packet.contains("#") {
            return Err(ParseError::MalformedPacket);
        }
        packet = packet
            .trim_start_matches("+")
            .trim_start_matches("-")
            .to_string();
        let fields: Vec<&str> = packet.split("#").collect();

        if fields.len() != 2 {
            eprintln!("Incorrect number of fields: {}", fields.len());
            return Err(ParseError::MalformedPacket);
        }
        let data_raw = fields[0];
        let checksum_raw = fields[1];

        match usize::from_str_radix(checksum_raw, 16) {
            Ok(request_checksum) => {
                let data_checksum: usize =
                    data_raw.chars().map(|x| x as usize).sum::<usize>() & 0xFF;

                if request_checksum != data_checksum {
                    eprintln!(
                        "Checksum mismatch: {} != {}",
                        request_checksum, data_checksum
                    );
                    return Err(ParseError::IncorrectChecksum);
                }
            }
            Err(e) => {
                eprintln!("Invalid checksum value: {}", checksum_raw);
                return Err(ParseError::MalformedPacket);
            }
        }

        Ok(RdbPacket {
            data: RdbPacket::parse_data_arguments(data_raw)?
                .iter()
                .map(|x| x.to_string())
                .collect(),
        })
    }
}
