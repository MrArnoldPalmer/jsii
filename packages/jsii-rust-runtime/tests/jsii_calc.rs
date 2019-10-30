// Example of generated rust code
pub mod base_of_base {
    use jsii_runtime::api::*;
    use jsii_runtime::{JsiiClient, JsiiClientError, JsiiObject};
    use std::convert::TryInto;

    pub trait JsiiVeryBaseInterface<'a>: JsiiObject<'a> {
        fn foo(&'a mut self) -> Result<(), JsiiClientError> {
            self.call_method("foo".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiVeryBaseProps<'a>: JsiiObject<'a> {
        fn get_foo(&mut self) -> Result<Very, JsiiClientError>;
    }

    pub trait JsiiVery<'a>: JsiiObject<'a> {
        fn hey(&'a mut self) -> Result<f64, JsiiClientError> {
            self.call_method("hey".into(), Vec::new())?.try_into()
        }
    }

    // Very
    pub struct Very<'a> {
        client: &'a mut JsiiClient,
        obj_ref: JsiiObjRef,
    }
    impl<'a> JsiiObject<'a> for Very<'_> {
        const FQN: &'a str = "@scope/jsii-calc-base-of-base.Very";
        fn get_ref(&self) -> JsiiObjRef {
            self.obj_ref.clone()
        }

        fn get_client(&'a mut self) -> &'a mut JsiiClient {
            self.client
        }
    }
    impl<'a> JsiiVery<'a> for Very<'_> {}
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
    }
}

pub mod base {
    use super::base_of_base::{JsiiVeryBaseInterface, JsiiVeryBaseProps};
    use jsii_runtime::{JsiiClientError, JsiiObject};
    use std::convert::TryInto;

    pub trait JsiiBase<'a>: JsiiObject<'a> {
        fn type_name(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("typeName".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiBaseProps<'a>: JsiiVeryBaseProps<'a> + JsiiObject<'a> {
        fn get_bar(&'a mut self) -> Result<String, JsiiClientError> {
            self.jsii_get_property("bar".into())?.try_into()
        }
    }

    pub trait JsiiBaseInterface<'a>: JsiiVeryBaseInterface<'a> {
        fn bar(&'a mut self) -> Result<(), JsiiClientError> {
            self.call_method("bar".into(), Vec::new())?.try_into()
        }
    }
}

pub mod lib {
    use super::base::{JsiiBase, JsiiBaseInterface};
    use jsii_runtime::api::*;
    use jsii_runtime::{JsiiClient, JsiiClientError, JsiiObject};
    use std::convert::TryInto;

    pub trait JsiiValue<'a>: JsiiBase<'a> {
        fn get_value(&'a mut self) -> Result<f64, JsiiClientError> {
            self.jsii_get_property("value".into())?.try_into()
        }

        fn to_string(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("toString".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiDoublable<'a>: JsiiObject<'a> {
        fn get_double_value(&'a mut self) -> Result<f64, JsiiClientError> {
            self.jsii_get_property("doubleValue".into())?.try_into()
        }
    }

    pub trait JsiiNumber<'a>: JsiiValue<'a> + JsiiDoublable<'a> + JsiiObject<'a> {}

    pub struct Number<'a> {
        client: &'a mut JsiiClient,
        obj_ref: JsiiObjRef,
    }

    impl<'a> JsiiBase<'a> for Number<'_> {}
    impl<'a> JsiiValue<'a> for Number<'_> {}
    impl<'a> JsiiDoublable<'a> for Number<'_> {}
    impl<'a> JsiiNumber<'a> for Number<'_> {}
    impl<'a> JsiiObject<'a> for Number<'_> {
        const FQN: &'a str = "@scope/jsii-calc-lib.Number";
        fn get_ref(&self) -> JsiiObjRef {
            self.obj_ref.clone()
        }

        fn get_client(&'a mut self) -> &'a mut JsiiClient {
            self.client
        }
    }

    impl<'a> Number<'a> {
        pub fn new(
            client: &'a mut JsiiClient,
            value: impl Into<f64>,
        ) -> Result<Self, JsiiClientError> {
            client
                .create_object(JsiiCreateObject {
                    fqn: Self::FQN.to_string(),
                    args: vec![serde_json::Value::from(value.into() as f64)],
                })
                .map(move |response| Self {
                    obj_ref: response,
                    client,
                })
        }
    }

    pub trait JsiiOperation<'a>: JsiiValue<'a> {
        fn to_string(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("toString".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiFriendly<'a>: JsiiObject<'a> {
        fn hello(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("hello".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiMyFirstStruct<'a>: JsiiObject<'a> {
        fn get_astring(&'a mut self) -> Result<String, JsiiClientError> {
            self.jsii_get_property("astring".into())?.try_into()
        }

        fn get_anumber(&'a mut self) -> Result<f64, JsiiClientError> {
            self.jsii_get_property("anumber".into())?.try_into()
        }

        fn get_first_optional(&'a mut self) -> Result<Option<Vec<String>>, JsiiClientError> {
            self.jsii_get_property("astring".into())?.try_into()
        }
    }

    pub trait JsiiStructWithOnlyOptionals<'a>: JsiiObject<'a> {
        fn get_optional1(&'a mut self) -> Result<Option<String>, JsiiClientError> {
            self.jsii_get_property("astring".into())?.try_into()
        }

        fn get_anumber(&'a mut self) -> Result<f64, JsiiClientError> {
            self.jsii_get_property("anumber".into())?.try_into()
        }

        fn get_first_optional(&'a mut self) -> Result<Option<Vec<String>>, JsiiClientError> {
            self.jsii_get_property("astring".into())?.try_into()
        }
    }

    // #[derive(Deserialize, Serialize)]
    pub enum EnumFroomScopedModule {
        Value1,
        Value2,
    }

    pub trait JsiiThreeLevelsInterface<'a>: JsiiBaseInterface<'a> {
        fn baz(&'a mut self) -> Result<(), JsiiClientError> {
            self.call_method("baz".into(), Vec::new())?.try_into()
        }
    }
}

pub mod calculator {
    use super::lib::{JsiiFriendly, JsiiNumber, JsiiOperation, JsiiValue};
    use jsii_runtime::{JsiiClientError, JsiiObject};
    use std::convert::TryInto;

    pub trait JsiiFriendlier<'a>: JsiiFriendly<'a> {
        fn goodbye(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("goodbye".into(), Vec::new())?.try_into()
        }

        fn farewell(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("farewell".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiRandomNumberGenerator<'a>: JsiiObject<'a> {
        fn next(&'a mut self) -> Result<f64, JsiiClientError> {
            self.call_method("next".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiFriendlyRandomGenerator<'a>:
        JsiiRandomNumberGenerator<'a> + JsiiFriendly<'a>
    {
    }

    pub trait JsiiBinaryOperation<'a>: JsiiOperation<'a> + JsiiFriendly<'a> {
        fn hello(&'a mut self) -> Result<String, JsiiClientError> {
            self.call_method("hello".into(), Vec::new())?.try_into()
        }
    }

    pub trait JsiiAdd<'a>: JsiiBinaryOperation<'a> {
        fn get_value(&'a mut self) -> Result<f64, JsiiClientError> {
            self.jsii_get_property("value".into())?.try_into()
        }

        fn to_string(&'a mut self) -> Result<f64, JsiiClientError> {
            self.call_method("toString".into(), Vec::new())?.try_into()
        }
    }
}

// TODO: Add modules where needed
// pub mod calculator;
// pub mod compliance;
// pub mod documented;
// pub mod erasures;
// pub mod stability;
