use base64::{Engine as _, engine::general_purpose::STANDARD};

pub fn encode_to_base64(input: &str) -> String {
    STANDARD.encode(input)
}

pub fn decode_from_base64(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(input)
}

pub fn process_args(args: &[String]) -> String {
    if is_arg_len_invalid(args) {
        println!("Invalid number of arguments. Ex: b64 e hello");
        return String::new();
    }

    let process = extract_process(args);
    let input = extract_input(args);

    let result = process_input(process, input);

    println!("{}", result);
    result
}

fn is_arg_len_invalid(args: &[String]) -> bool {
    args.len() > 3 || args.len() < 2
}

fn extract_process(args: &[String]) -> &str {
    args.get(1)
        .expect("Missing process please enter encode or decode, or e or d. Ex: b64 e hello")
        .trim()
}

fn extract_input(args: &[String]) -> &str {
    args.get(2)
        .expect("Missing input please enter a string to encode or decode")
        .trim()
}

fn process_input(process: &str, input: &str) -> String {
    if process == "decode" || process == "d" {
        return match decode_from_base64(input) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
            Err(err) => {
                println!("Error: {}", err);
                panic!("Invalid base64 input");
            }
        };
    } else if process == "encode" || process == "e" {
        return encode_to_base64(input);
    }
    eprintln!("Invalid process");
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "hello";
        let output = encode_to_base64(input);
        assert_eq!(output, "aGVsbG8=");
    }

    #[test]
    fn test_decode() {
        let input = "aGVsbG8=";
        let output = decode_from_base64(input).unwrap();
        assert_eq!(String::from_utf8_lossy(&output), "hello");
    }

    #[test]
    fn test_decode_error() {
        let input = "aGVsbG8";
        let output = decode_from_base64(input);
        assert!(output.is_err());
    }
    #[test]
    fn test_is_arg_len_invalid() {
        let args = vec![
            "hello".to_string(),
            "yo".to_string(),
            "b".to_string(),
            "s".to_string(),
        ];
        assert!(is_arg_len_invalid(&args));
    }

    #[test]
    fn test_extract_process() {
        let args = vec!["b64".to_string(), "encode".to_string(), "hello".to_string()];
        let output = extract_process(&args);
        assert_eq!(output, "encode");
    }

    #[test]
    #[should_panic(
        expected = "Missing process please enter encode or decode, or e or d. Ex: b64 e hello"
    )]
    fn test_extract_process_error() {
        let args = vec!["hello".to_string()];
        let output = extract_process(&args);
        assert_eq!(output, "invalid");
    }

    #[test]
    fn test_extract_input() {
        let args = vec!["b64".to_string(), "encode".to_string(), "hello".to_string()];
        let output = extract_input(&args);
        assert_eq!(output, "hello");
    }

    #[test]
    #[should_panic(expected = "Missing input please enter a string to encode or decode")]
    fn test_extract_input_error() {
        let args = vec!["b64".to_string(), "encode".to_string()];
        extract_input(&args);
    }
}
