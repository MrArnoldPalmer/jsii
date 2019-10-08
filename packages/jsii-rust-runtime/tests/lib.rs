use jsii_runtime::{JsiiClient, JsiiModule};

#[test]
fn load_process() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    let mut client = JsiiClient::new(None).expect("Error creating jsii client");

    // Run `cargo make pre_test` to generate module tarballs
    let jsii_calc_base = JsiiModule {
        name: "@scope/jsii-calc-base".into(),
        version: "0.17.0".into(),
        tarball: format!(
            "{}/target/jsii-calc/js/jsii-calc-base@0.17.0.jsii.tgz",
            root_dir
        ),
    };

    let jsii_calc_lib = JsiiModule {
        name: "@scope/jsii-calc-lib".into(),
        version: "0.17.0".into(),
        tarball: format!(
            "{}/target/jsii-calc/js/jsii-calc-lib@0.17.0.jsii.tgz",
            root_dir
        ),
    };

    let jsii_calc = JsiiModule {
        name: "@scope/jsii-calc".into(),
        version: "0.17.0".into(),
        tarball: format!("{}/target/jsii-calc/js/jsii-calc@0.17.0.jsii.tgz", root_dir),
    };

    let base_response = dbg!(client.load_module(jsii_calc_base));
    let lib_response = dbg!(client.load_module(jsii_calc_lib));
    let response = dbg!(client.load_module(jsii_calc));
    assert!(
        base_response.is_ok(),
        "Err loading module {:?}",
        base_response
    );
    assert!(
        lib_response.is_ok(),
        "Err loading module {:?}",
        lib_response
    );
    assert!(response.is_ok(), "Err loading module {:?}", response);
}
