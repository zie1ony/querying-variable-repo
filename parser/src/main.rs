use std::env;

use casper_types::{bytesrepr::{ToBytes, FromBytes, Bytes}, U256};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

// First value - actual value in bytes.
// Second value - future value, activation time.
 
type Record = (Bytes, Option<(Bytes, u64)>);

fn to_dictionary_item_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap();
    let hash = blake2b(preimage);
    hex::encode(hash)
}

fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut result = [0; 32];
    let mut hasher = VarBlake2b::new(32).expect("should create hasher");

    hasher.update(data);
    hasher.finalize_variable(|slice| {
        result.copy_from_slice(slice);
    });
    result
}

fn assert_is_empty(bytes: &[u8]) {
    if !bytes.is_empty() {
        panic!("bytes not empty: {:?}", bytes);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        "to-dictionary-item-key-string" => {
            let index: String = args[2].parse().unwrap();
            let result = to_dictionary_item_key(&index);
            println!("{}", result);
        },
        "decode-repo-value" => {
            let param_name: String = args[2].parse().unwrap();
            let bytes: Vec<u8> = hex::decode(&args[3]).unwrap();
            
            let (record, bytes): (Record, &[u8]) = FromBytes::from_bytes(&bytes).unwrap();
            assert_is_empty(bytes);
            
            let (value, _future_value) = record;
            match param_name.as_str() {
                "default_policing_rate" => {
                    let (value, bytes) = U256::from_bytes(&value).unwrap();
                    assert_is_empty(bytes);
                    println!("{}: {}", param_name, value);
                },
                "forum_kyc_required" => {
                    let (value, bytes) = bool::from_bytes(&value).unwrap();
                    assert_is_empty(bytes);
                    println!("{}: {}", param_name, value);
                }
                _ => panic!("Unsupported parameter {}", param_name)
            }
        }

        _ => panic!("Unknown command: {}", command)
    }
}
