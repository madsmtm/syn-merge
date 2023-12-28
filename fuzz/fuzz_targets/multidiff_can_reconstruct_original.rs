#![no_main]
libfuzzer_sys::fuzz_target!(
    |data: (Vec<u8>, Vec<u8>, Vec<u8>)| fuzz::can_reconstruct_original(data)
);
