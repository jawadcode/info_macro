pub trait InfoMacro {
    fn info(&self);
}

#[test]
fn test() {
    use info_macro_derive::InfoMacro;

    #[derive(InfoMacro)]
    struct Thing;

    let thing = Thing;
    thing.info();
}
