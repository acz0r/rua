#[test]
fn local_var() {
    super::assert_transpiles_to("local_var", "simple");
}
