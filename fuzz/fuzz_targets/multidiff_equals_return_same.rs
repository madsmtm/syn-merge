#![no_main]
libfuzzer_sys::fuzz_target!(|data: Vec<u8>| fuzz::equals_return_same(data));
