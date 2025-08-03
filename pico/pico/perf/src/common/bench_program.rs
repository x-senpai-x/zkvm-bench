use anyhow::Error;
use bincode::de;
use pico_vm::emulator::stdin::EmulatorStdin;
use snap::raw::Decoder;
use std::fs;

#[derive(Clone, Copy)]
pub struct BenchProgram {
    pub name: &'static str,
    pub elf: &'static str,
    pub input: Option<&'static str>,
    pub operation_input: Option<&'static str>,
}

pub const PROGRAMS: &[BenchProgram] = &[
    // BenchProgram {
    //     name: "fibonacci-300kn",
    //     elf: "./perf/bench_data/fibonacci-elf",
    //     input: Some("fibonacci-300kn"),
    // },
    // BenchProgram {
    //     name: "tendermint",
    //     elf: "./perf/bench_data/tendermint-elf",
    //     input: None,
    // },
    // BenchProgram {
    //     name: "reth-17106222",
    //     elf: "./perf/bench_data/reth-elf",
    //     input: Some("./perf/bench_data/reth-17106222.bin"),
    // },
    // BenchProgram {
    //     name: "reth-22059900",
    //     elf: "./perf/bench_data/reth-elf",
    //     input: Some("./perf/bench_data/reth-22059900.bin"),
    // },
    // BenchProgram {
    //     name: "reth-20528709",
    //     elf: "./perf/bench_data/reth-elf",
    //     input: Some("./perf/bench_data/reth-20528709.bin"),
    // },
    BenchProgram {
        name: "ream-pico",
        elf: "./perf/bench_data/riscv32im-pico-zkvm-elf",
        input: Some("./perf/bench_data/at_max_inclusion_slot/pre.ssz_snappy"),
        operation_input: None, // Temporarily disable the second input
    },
];

#[allow(clippy::type_complexity)]
pub fn load<P>(bench: &BenchProgram) -> Result<(Vec<u8>, EmulatorStdin<P, Vec<u8>>), Error> {
    let elf = std::fs::read(bench.elf)?;
    let mut stdin_builder = EmulatorStdin::<P, Vec<u8>>::new_builder();
    if let Some(operation_input) = bench.operation_input {
        let input_bytes = fs::read(operation_input)?; //gives raw bytes
        let mut decoder = Decoder::new();
        let decoded_input = decoder
            .decompress_vec(&input_bytes)
            .map_err(|e| Error::msg(format!("Failed to decompress operation_input: {}", e)))?;
        println!(
            "Decompressed operation_input: {} bytes",
            decoded_input.len()
        );
        stdin_builder.write_slice(&decoded_input);
    }
    if let Some(input) = bench.input {
        let input_bytes = fs::read(input)?; //gives raw bytes
        let mut decoder = Decoder::new();
        let decoded_input = decoder
            .decompress_vec(&input_bytes)
            .map_err(|e| Error::msg(format!("Failed to decompress input: {}", e)))?;
        println!("Decompressed input: {} bytes", decoded_input.len());
        stdin_builder.write_slice(&decoded_input);
    }

    Ok((elf, stdin_builder.finalize()))
}
