// Example of generated rust code
pub mod base_of_base {
    use jsii_runtime::api::*;
    use jsii_runtime::{JsiiClient, JsiiClientError, JsiiObject};
    use std::convert::TryInto;

    pub trait VeryBaseInterface {
        fn foo();
    }

    #[derive(Debug)]
    pub struct VeryBaseProps<'a> {
        foo: Very<'a>,
    }

    #[derive(Debug)]
    pub struct Very<'a> {
        client: &'a mut JsiiClient,
        obj_ref: ObjRef,
    }

    impl<'a> JsiiObject<'a> for Very<'_> {
        const FQN: &'a str = "@scope/jsii-calc-base-of-base.Very";
        fn get_ref(&self) -> ObjRef {
            self.obj_ref.clone()
        }

        fn get_client(&'a mut self) -> &'a mut JsiiClient {
            self.client
        }
    }

    impl<'a> Very<'a> {
        pub fn new(client: &'a mut JsiiClient) -> Result<Self, JsiiClientError> {
            client
                .create_object(JsiiCreateObject {
                    fqn: Self::FQN.to_string(),
                    args: Vec::new(),
                })
                .map(move |response| Self {
                    obj_ref: response,
                    client,
                })
        }

        pub fn hey(&mut self) -> Result<f64, JsiiClientError> {
            self.call_method("hey".into(), Vec::new())?.try_into()
        }
    }
}

pub mod base {
    use super::base_of_base::{VeryBaseInterface, VeryBaseProps};

    pub struct Base {}
    impl Base {
        pub fn type_name() -> String {
            "type_name".into()
        }
    }
}
