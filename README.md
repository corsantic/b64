# b64

A simple and fast command-line tool for encoding and decoding base64 strings.

## Installation

```bash
cargo install --path .
```

## Usage

The basic syntax is:

```bash
b64 <command> <input>
```

### Commands

- `encode` or `e` - Encode a string to base64
- `decode` or `d` - Decode a base64 string

### Examples

**Encoding:**
```bash
b64 encode hello
# Output: aGVsbG8=

b64 e hello
# Output: aGVsbG8=
```

**Decoding:**
```bash
b64 decode aGVsbG8=
# Output: hello

b64 d aGVsbG8=
# Output: hello
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

MIT
