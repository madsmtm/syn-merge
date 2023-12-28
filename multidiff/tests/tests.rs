use multidiff::multidiff;

fn assert_diff_matches(data: &[&str], expected: &[(char, &[usize])]) {
    let data: Vec<Vec<char>> = data.iter().map(|s| s.chars().collect()).collect();
    let data: Vec<&[char]> = data.iter().map(|chars| &**chars).collect();
    let actual: Vec<_> = multidiff(&data);
    assert_eq!(actual.len(), expected.len());

    for (i, (expected, actual)) in expected.iter().zip(actual).enumerate() {
        assert_eq!(expected.0, *actual.0);
        assert_eq!(expected.1, actual.1 .0, "idx: {i}");
    }
}

#[test]
fn empty() {
    assert_diff_matches(&[], &[]);
}

#[test]
fn equals() {
    assert_diff_matches(&["ab", "ab", "ab"], &[('a', &[0, 1, 2]), ('b', &[0, 1, 2])]);
}

#[test]
fn different() {
    assert_diff_matches(&["a", "b", "c"], &[('a', &[0]), ('b', &[1]), ('c', &[2])]);
}

#[test]
fn one_differs() {
    assert_diff_matches(
        &["ab", "ab", "ac"],
        &[('a', &[0, 1, 2]), ('b', &[0, 1]), ('c', &[2])],
    );
}

#[test]
fn one() {
    assert_diff_matches(
        &["abbc"],
        &[('a', &[0]), ('b', &[0]), ('b', &[0]), ('c', &[0])],
    );
}

#[test]
fn two() {
    assert_diff_matches(
        &["aaabbbccc", "baacccc"],
        &[
            ('b', &[1]),
            ('a', &[0, 1]),
            ('a', &[0, 1]),
            ('a', &[0]),
            ('b', &[0]),
            ('b', &[0]),
            ('b', &[0]),
            ('c', &[1]),
            ('c', &[0, 1]),
            ('c', &[0, 1]),
            ('c', &[0, 1]),
        ],
    );
}

#[test]
fn three() {
    assert_diff_matches(
        &["abc", "bac", "bca"],
        &[
            ('a', &[0]),
            ('b', &[0, 1, 2]),
            ('a', &[1]),
            ('c', &[0, 1, 2]),
            ('a', &[2]),
        ],
    );
}

// https://github.com/mitsuhiko/similar/issues/57
#[test]
fn avoids_similar_crash() {
    let _ = multidiff(&[&[0], &[0, 0]]);
}
