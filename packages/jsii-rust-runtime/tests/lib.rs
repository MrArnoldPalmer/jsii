use jsii_rust_runtime::{JsiiModule, JsiiRuntime};

#[test]
fn load_process() {
    let runtime = JsiiRuntime::new().expect("Error creating runtime");

    let module = JsiiModule {
        name: "jsii-calc".into(),
        version: "0.17.0".into(),
    };

    let response = runtime.load_module(module).expect("Can't get response");
    assert_eq!(response["hello"].as_str().unwrap(), "jsii-runtime@0.17.0");
}
