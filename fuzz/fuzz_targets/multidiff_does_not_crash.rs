#![no_main]
libfuzzer_sys::fuzz_target!(|data: (Vec<u8>, Vec<u8>, Vec<u8>)| fuzz::does_not_crash(data));
