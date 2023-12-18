use crate::*;
use proc_macro2::TokenStream;
use quote::quote;

fn assert_merged(files: &[(TokenStream, &str)], expected: TokenStream) {
    let expected = syn::parse_file(&expected.to_string()).unwrap();

    let files: Vec<_> = files
        .iter()
        .map(|(tokens, cfgs)| {
            (
                syn::parse_file(&tokens.to_string()).unwrap(),
                Cfgs::new(cfgs),
            )
        })
        .collect();
    let actual = merge_files(&files).unwrap();
    if expected != actual {
        panic!(
            "expected:\n\n{}\n-------\nactual:\n\n{}",
            prettyplease::unparse(&expected),
            prettyplease::unparse(&actual),
        );
    }
}

macro_rules! files_with_cfg {
    (
        $(
            #[cfg($cfg:ident)]
            mod _ {
                $($stmt:stmt)*
            }
        )*
    ) => {
        &[$(
            (quote!($($stmt)*), stringify!($cfg)),
        )*]
    };
}

#[test]
fn simple_function() {
    assert_merged(
        files_with_cfg! {
            #[cfg(foo)]
            mod _ {
                fn foo() {}
                fn foobar() {}
            }

            #[cfg(bar)]
            mod _ {
                fn bar() {}
                fn foobar() {}
            }
        },
        quote! {
            #[cfg(foo)]
            fn foo() {}
            #[cfg(bar)]
            fn bar() {}
            fn foobar() {}
        },
    );
}

#[test]
fn differing_functions() {
    assert_merged(
        files_with_cfg! {
            #[cfg(foo)]
            mod _ {
                fn func() {
                    let foo = 5;
                }
            }

            #[cfg(bar)]
            mod _ {
                fn func() {
                    let bar = 5;
                }
            }
        },
        quote! {
            #[cfg(foo)]
            fn func() {
                let foo = 5;
            }
            #[cfg(bar)]
            fn func() {
                let bar = 5;
            }
        },
    );
}

#[test]
fn swapping() {
    assert_merged(
        files_with_cfg! {
            #[cfg(a)]
            mod _ {
                const FOO1: usize = 1;
                const FOO2: usize = 2;
            }

            #[cfg(b)]
            mod _ {
                const FOO2: usize = 2;
                const FOO1: usize = 1;
            }
        },
        quote! {
            #[cfg(a)]
            const FOO1: usize = 1;
            const FOO2: usize = 2;
            #[cfg(b)]
            const FOO1: usize = 1;
        },
    );
}
