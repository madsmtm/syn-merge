use multidiff::multidiff;

pub fn does_not_crash(data: (Vec<u8>, Vec<u8>, Vec<u8>)) {
    let _ = multidiff([&data.0, &data.1, &data.2]);
}

pub fn equals_return_same(data: Vec<u8>) {
    let res = multidiff([&data, &data, &data]);
    assert_eq!(res.len(), data.len());
    for (input, output) in res.iter().zip(&data) {
        assert_eq!(input.0, *output);
    }
}

pub fn can_reconstruct_original((a, b, c): (Vec<u8>, Vec<u8>, Vec<u8>)) {
    let res = multidiff([&a, &b, &c]);

    let orig_a: Vec<_> = res
        .iter()
        .filter(|chunk| chunk.1.contains(0))
        .map(|chunk| chunk.0)
        .collect();
    assert_eq!(orig_a, a);

    let orig_b: Vec<_> = res
        .iter()
        .filter(|chunk| chunk.1.contains(1))
        .map(|chunk| chunk.0)
        .collect();
    assert_eq!(orig_b, b);

    let orig_c: Vec<_> = res
        .iter()
        .filter(|chunk| chunk.1.contains(2))
        .map(|chunk| chunk.0)
        .collect();
    assert_eq!(orig_c, c);
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_not_crash() {
        quickcheck::quickcheck(super::does_not_crash as fn(_) -> _);
    }

    #[test]
    fn equals_return_same() {
        quickcheck::quickcheck(super::equals_return_same as fn(_) -> _);
    }

    #[test]
    fn can_reconstruct_original() {
        quickcheck::quickcheck(super::can_reconstruct_original as fn(_) -> _);
    }
}
