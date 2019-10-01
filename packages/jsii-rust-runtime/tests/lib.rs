use jsii_rust_runtime::{JsiiModule, JsiiRuntime};

#[test]
fn load_process() {
    let mut runtime = JsiiRuntime::new().expect("Error creating runtime");

    let module = JsiiModule {
        name: "jsii-calc".into(),
        version: "0.17.0".into(),
    };

    let response = runtime.load_module(module);
    assert!(response.is_ok(), "Err loading module {:?}", response);
}
