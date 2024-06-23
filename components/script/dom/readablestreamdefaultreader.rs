/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::jsapi::{
    AutoRequireNoGC, HandleObject, HandleValue, Heap, IsReadableStream, JSContext, JSObject,
};
use js::jsval::{JSVal, ObjectValue, UndefinedValue};
use js::rust::{HandleObject as SafeHandleObject, HandleValue as SafeHandleValue, IntoHandle};

use crate::dom::bindings::codegen::Bindings::ReadableStreamDefaultReaderBinding::ReadableStreamDefaultReaderMethods;
use crate::dom::bindings::conversions::{ConversionBehavior, ConversionResult};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::import::module::Fallible;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::settings_stack::{AutoEntryScript, AutoIncumbentScript};
use crate::dom::bindings::utils::get_dictionary_property;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::Promise;
use crate::dom::readablestream::ReadableStream;
use crate::js::conversions::FromJSValConvertible;
use crate::realms::{enter_realm, InRealm};
use crate::script_runtime::JSContext as SafeJSContext;

#[dom_struct]
pub struct ReadableStreamDefaultReader {
    reflector_: Reflector,
}

impl ReadableStreamDefaultReader {
    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<SafeHandleObject>,
        stream: DomRoot<ReadableStream>,
    ) -> Fallible<DomRoot<Self>> {
        todo!()
    }

    fn new_inherited() -> ReadableStreamDefaultReader {
        ReadableStreamDefaultReader {
            reflector_: Reflector::new(),
        }
    }

    fn new(global: &GlobalScope) -> DomRoot<ReadableStreamDefaultReader> {
        reflect_dom_object(
            Box::new(ReadableStreamDefaultReader::new_inherited()),
            global,
        )
    }
}

impl ReadableStreamDefaultReaderMethods for ReadableStreamDefaultReader {
    fn Read(&self) -> std::rc::Rc<Promise> {
        todo!()
    }

    fn ReleaseLock(&self) -> Fallible<()> {
        todo!()
    }

    fn Closed(&self) -> std::rc::Rc<Promise> {
        todo!()
    }

    fn Cancel(&self, cx: SafeJSContext, reason: SafeHandleValue) -> std::rc::Rc<Promise> {
        todo!()
    }
}
