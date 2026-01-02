use b64::{decode_from_base64, encode_to_base64, process_args};

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
fn test_process_input_encode() {
    let args = vec!["b64".to_string(), "encode".to_string(), "hello".to_string()];
    assert_eq!(process_args(&args), "aGVsbG8=");
}

#[test]
fn test_process_input_decode() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "aGVsbG8=".to_string(),
    ];
    assert_eq!(process_args(&args), "hello");
}

#[test]
fn test_process_input_decode_with_spaces() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "aGVsbG8= ".to_string(),
    ];
    assert_eq!(process_args(&args), "hello");
}

#[test]
fn test_process_input_decode_with_newline() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "aGVsbG8=\n".to_string(),
    ];
    assert_eq!(process_args(&args), "hello");
}

#[test]
fn test_process_input_decode_with_tab() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "aGVsbG8=\t".to_string(),
    ];
    assert_eq!(process_args(&args), "hello");
}

#[test]
fn test_process_input_decode_with_carriage_return() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "aGVsbG8=\r".to_string(),
    ];
    assert_eq!(process_args(&args), "hello");
}

#[test]
#[should_panic(expected = "Invalid base64 input")]
fn test_process_input_decode_with_invalid_base64() {
    let args = vec![
        "b64".to_string(),
        "decode".to_string(),
        "M".to_string(),
    ];
    process_args(&args);
}

