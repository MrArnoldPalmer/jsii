use jsii_rust_runtime::{JsiiModule, JsiiRuntime};

#[test]
fn load_process() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    let mut runtime = JsiiRuntime::new().expect("Error creating runtime");

    let module = JsiiModule {
        name: "jsii-calc".into(),
        version: "0.17.0".into(),
        // Run cargo make pre_test to generate jsii-calc tarball
        tarball: format!("{}/jsii-calc.tar.gz", root_dir),
    };

    let response = runtime.load_module(module);
    assert!(response.is_ok(), "Err loading module {:?}", response);
    dbg!(&response);
}
