pub trait App {
    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn powered_desc(&self) -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }
    fn powered_ver(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
