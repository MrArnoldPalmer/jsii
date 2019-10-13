use crate::api::ObjRef;

pub trait JsiiObject {
    fn get_fqn() -> String;
    fn get_ref(self) -> ObjRef;
}
