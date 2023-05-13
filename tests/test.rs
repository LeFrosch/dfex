use dfx::dfa;

dfa!(rx = "(a|b)*a(a|b)");

#[test]
fn test() {
    
}