use jsii_runtime::api::*;
use jsii_runtime::{JsiiClient, JsiiClientError, JsiiModule, JsiiObject};
use std::convert::TryInto;

// Example of generated rust code
#[derive(Debug)]
struct Very<'a> {
    client: &'a mut JsiiClient,
    obj_ref: ObjRef,
}

impl<'a> JsiiObject<'a> for Very<'_> {
    fn get_fqn() -> String {
        String::from("@scope/jsii-calc-base-of-base.Very")
    }

    fn get_ref(&self) -> ObjRef {
        self.obj_ref.clone()
    }

    fn get_client(&'a mut self) -> &'a mut JsiiClient {
        self.client
    }
}

impl<'a> Very<'a> {
    fn new(client: &'a mut JsiiClient) -> Result<Self, JsiiClientError> {
        client
            .create_object(JsiiCreateObject {
                fqn: Self::get_fqn(),
                args: Vec::new(),
            })
            .map(move |response| Self {
                obj_ref: response,
                client,
            })
    }

    fn hey(&mut self) -> Result<f64, JsiiClientError> {
        let obj_ref = self.get_ref();
        self.get_client()
            .call_method(JsiiInvokeRequest {
                obj_ref,
                method: "hey".into(),
                args: Vec::new(),
            })?
            .try_into()
    }
}

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

    let mut very = dbg!(Very::new(&mut client).expect("Can't create very instance"));
    dbg!(very.hey().expect("Error calling method on Very instance"));
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
