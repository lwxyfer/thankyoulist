# thankyoulist

[![Crates.io]()](https://crates.io/crates/thankyoulist)[![License]()](https://github.com/your-username/thankyoulist/blob/main/LICENSE)

thankyoulist is a Rust script that generates a JSON file listing the open-source projects your project depends on.

## Installation

Make sure you have Rust and Cargo installed. Then, you can install thankyoulist by running the following command:

```bash
cargo install thankyoulist
```

## USAGE

To use thankyoulist, navigate to your project directory and run the following command:

```
thankyoulist
```

## Example Output 

```
[
  {
    "name": "lazy_static",
    "version": "1.4.0",
    "description": "A macro for declaring lazily evaluated statics in Rust.",
    "url": "",
    "license": ""
  },
  {
    "name": "reqwest",
    "version": "0.11",
    "description": "higher level HTTP client library",
    "url": "",
    "license": ""
  },
  {
    "name": "serde",
    "version": "1.0",
    "description": "A generic serialization/deserialization framework",
    "url": "https://serde.rs",
    "license": ""
  },
  {
    "name": "serde_derive",
    "version": "1.0",
    "description": "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]",
    "url": "https://serde.rs",
    "license": ""
  },
  {
    "name": "serde_json",
    "version": "1.0",
    "description": "A JSON serialization file format",
    "url": "",
    "license": ""
  }
]
```