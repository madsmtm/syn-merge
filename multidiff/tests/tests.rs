use multidiff::multidiff;

fn assert_diff_matches(slice: &[&[u8]], expected: &[(u8, &[usize])]) {
    let actual: Vec<_> = multidiff(slice.iter().map(AsRef::as_ref)).collect();
    assert_eq!(actual.len(), expected.len());

    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(expected.0, *actual.value);
        assert_eq!(expected.1, actual.appears_in);
    }
}

#[test]
fn empty() {
    assert_diff_matches(&[], &[]);
}

#[test]
fn single() {
    assert_diff_matches(
        &[b"abbc"],
        &[(b'a', &[0]), (b'b', &[0]), (b'b', &[0]), (b'c', &[0])],
    );
}

#[test]
fn simple() {
    assert_diff_matches(
        &[b"abc", b"bac", b"bca"],
        &[
            (b'a', &[0]),
            (b'b', &[0, 1, 2]),
            (b'a', &[1]),
            (b'c', &[0, 1, 2]),
            (b'a', &[2]),
        ],
    );
}
