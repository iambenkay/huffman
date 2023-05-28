use std::cmp::min;

#[derive(Debug)]
pub enum Error {
    NotValid
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn bytes_to_bit_str(bytes: Vec<u8>) -> String {
    let mut bits = String::new();


    for byte  in bytes {
        let bin_repr = format!("{:b}", byte);
        bits.push_str(&bin_repr.replace("0b", ""));
    }
    bits
}

pub fn bit_str_to_bytes(bits: &str) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut i = 0;

    while i < bits.len() {
        let bin_str = &bits[i..min(i + 8, bits.len())];
        let byte = u8::from_str_radix(bin_str, 2).map_err(|_| Error::NotValid)?;
        bytes.push(byte);
        i += 8;
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bit_str_to_bytes_returns_238_7_when_passed_1110111010() {
        let bits = "11101110111";
        let bytes = super::bit_str_to_bytes(&bits);

        assert!(bytes.is_ok());
        if let Ok(bytes) = bytes {
            assert_eq!(bytes, vec![238, 7]);
        }
    }

    #[test]
    fn test_bit_str_to_bytes_returns_err_when_invalid_value_passed() {
        let bits = "2b";
         let bytes = super::bit_str_to_bytes(&bits);

         assert!(bytes.is_err())
    }

    #[test]
    fn test_bytes_to_bit_str_returns_11101110111_when_passed_238_7() {
        let bytes = vec![238, 7];
        let bits = super::bytes_to_bit_str(bytes);
        assert_eq!(bits, "11101110111");
    }
}