#![no_main]

pico_sdk::entrypoint!(main);
use ream_consensus::{attestation::Attestation, deneb::beacon_state::BeaconState};
use ssz::DecodeError;
struct ReaderDeneb{
    operation_input: Attestation,
    beacon_state: BeaconState,
}
pub fn main() {
    // Test with just one input first
    let state_input: Vec<u8> = pico_sdk::io::read_vec(); // Beacon state only
    
    println!("State input size: {} bytes", state_input.len());

    let mut pre_state: BeaconState = deserialize(&state_input);
    
    // Skip attestation processing for now
    println!("Successfully loaded BeaconState!");
    //we will be using process_attestation

    let _ = pre_state.process_attestation(&attestation);
    // Execute the block.

    // Commit the block hash.
    pico_sdk::io::commit(&pre_state);
}
pub fn from_ssz_bytes<T: ssz::Decode>(ssz_bytes: &[u8]) -> Result<T, DecodeError> {
    T::from_ssz_bytes(ssz_bytes)
}
pub fn deserialize<T: ssz::Decode>(ssz_bytes: &[u8]) -> T {
    println!("Attempting to deserialize {} bytes", ssz_bytes.len());
    if ssz_bytes.len() >= 32 {
        println!("First 32 bytes: {:?}", &ssz_bytes[..32]);
    } else {
        println!("All {} bytes: {:?}", ssz_bytes.len(), ssz_bytes);
    }
    let deserialized: T = from_ssz_bytes(&ssz_bytes).unwrap();
    deserialized
}
