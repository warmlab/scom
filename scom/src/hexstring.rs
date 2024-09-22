pub trait HexString {
    fn to_hex(&self) -> String;
    fn from_hex(hex: &str) -> Result<Vec<u8>, String>;
}

impl HexString for String {
    // Convert the string to a hex string with space between every two characters
    fn to_hex(&self) -> String {
        self.as_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")  // Add a space between every byte
    }

    // Convert a hex string back to a regular string
    fn from_hex(hex: &str) -> Result<Vec<u8>, String> {
        // Remove spaces from the hex string for conversion
        let hex = hex.split_whitespace();

        //let bytes_result = hex.map(|x| u8::from_str_radix(x, 16)).collect();
        let mut result = Vec::new();
        for s in hex {
            match u8::from_str_radix(s, 16) {
                Ok(byte) => result.push(byte),
                Err(err) => {
                    eprintln!("parse {} to byte in error: {}", s, err);
                    return Err(err.to_string());
                }
            }
        }


        Ok(result)

        /*
        if hex.len() % 2 != 0 {
            return Err("Invalid hex string length".to_string());
        }

        let bytes_result: Result<Vec<u8>, _> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
            .collect();

        match bytes_result {
            Ok(bytes) => bytes,//Ok(String::from_utf8(bytes).expect("Invalid UTF-8 sequence")),
            Err(_) => println!("Failed to convert hex string to bytes".to_string()),
        }
        */
    }
}
