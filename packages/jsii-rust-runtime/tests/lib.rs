use jsii_runtime::{JsiiClient, JsiiModule};

mod jsii_calc;
use jsii_calc::base_of_base::Very;

#[test]
fn load_process() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    let mut client = JsiiClient::new(None).expect("Error creating jsii client");

    // Run `cargo make pre_test` to generate module tarballs
    let jsii_calc_base_of_base = JsiiModule {
        name: "@scope/jsii-calc-base-of-base".into(),
        version: "0.17.0".into(),
        tarball: format!(
            "{}/target/jsii-calc/js/jsii-calc-base-of-base@0.17.0.jsii.tgz",
            root_dir
        ),
    };

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

    let base_of_base_response = client.load_module(jsii_calc_base_of_base);
    let base_response = client.load_module(jsii_calc_base);
    let lib_response = client.load_module(jsii_calc_lib);
    let response = client.load_module(jsii_calc);

    let very = Very::new(&mut client);
    assert!(very.is_ok(), "Err creating instance of Very");

    let hey_result = very.unwrap().hey().expect("Error calling hey method");
    // checking floating point equality though not ideal
    assert_eq!(hey_result, 42.0 as f64);

    assert!(
        base_of_base_response.is_ok(),
        "Err loading module {:?}",
        base_of_base_response
    );
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
