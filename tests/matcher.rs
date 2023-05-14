use dfx::dfa;

macro_rules! matcher_tests {
    ($($pattern:literal: { success: [$($success:literal),+] error: [$($error:literal),+] }),+) => {
        #[test]
        fn matcher_tests() {
            $({
                dfa!(rx = $pattern);

                $(
                    assert_eq!(rx($success), true, "regex \"{}\" should match \"{}\"", $pattern, $success);
                )+

                $(
                    assert_eq!(rx($error), false, "regex \"{}\" should not match \"{}\"", $pattern, $error);
                )+
            })+
        }
    };
}

matcher_tests!(
    "(a|b)*a(a|b)": {
        success: [ "aaa", "ab" ]
        error:   [ "aba" ]
    },
    "a*b": {
        success: [ "b", "ab", "aaaaaab" ]
        error:   [ "a", "aba"]
    }
);