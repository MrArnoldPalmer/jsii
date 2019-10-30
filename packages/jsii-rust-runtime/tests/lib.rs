use jsii_runtime::{JsiiClient, JsiiModule};

mod jsii_calc;
use jsii_calc::base::JsiiBase;
use jsii_calc::base_of_base::{JsiiVery, Very};
use jsii_calc::lib::{JsiiDoublable, Number};

fn load_modules(client: &mut JsiiClient) {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");

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

    client
        .load_module(jsii_calc_base_of_base)
        .expect("Can't load base of base module");
    client
        .load_module(jsii_calc_base)
        .expect("Can't load base module");
    client
        .load_module(jsii_calc_lib)
        .expect("Can't load lib module");
    client
        .load_module(jsii_calc)
        .expect("Can't load jsii calc module");
}

#[test]
fn very() {
    let mut client = JsiiClient::new(None).expect("Error creating jsii client");
    load_modules(&mut client);

    let very = Very::new(&mut client);
    assert!(very.is_ok(), "Err creating instance of Very");

    let hey_result = very.unwrap().hey().expect("Error calling hey method");
    assert_eq!(hey_result as i64, 42 as i64);
}

#[test]
fn number() {
    let mut client = JsiiClient::new(None).expect("Error creating jsii client");
    load_modules(&mut client);

    let mut number = Number::new(&mut client, 128).expect("Error creating instance of Number");
    let type_name_result = number.type_name().expect("Error calling typeName method");
    assert_eq!(&type_name_result, "Number");

    let double_result = number
        .get_double_value()
        .expect("Error accessing doubleValue");
    assert_eq!(double_result as i64, 128 * 2);
}
