/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::jsapi::{
    AutoRequireNoGC, HandleObject, HandleValue, Heap, IsReadableStream, JSContext, JSObject,
};
use js::jsval::{JSVal, ObjectValue, UndefinedValue};
use js::rust::{HandleObject as SafeHandleObject, HandleValue as SafeHandleValue, IntoHandle};

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::FunctionBinding::Function;
use crate::dom::bindings::codegen::Bindings::ReadableStreamBinding::ReadableStreamReader;
use crate::dom::bindings::codegen::Bindings::ReadableStreamDefaultReaderBinding::ReadableStreamDefaultReaderMethods;
use crate::dom::bindings::conversions::{ConversionBehavior, ConversionResult};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::import::module::Fallible;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::settings_stack::{AutoEntryScript, AutoIncumbentScript};
use crate::dom::bindings::utils::get_dictionary_property;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::Promise;
use crate::dom::readablestream::{ReadableStream, StreamState};
use crate::js::conversions::FromJSValConvertible;
use crate::realms::{enter_realm, InRealm};
use crate::script_runtime::JSContext as SafeJSContext;

#[dom_struct]
pub struct ReadableStreamDefaultReader {
    reflector_: Reflector,
    read_requests: DomRefCell<Vec<ReadRequest>>,
    stream: Option<DomRoot<ReadableStream>>,
    #[ignore_malloc_size_of = "promises are hard"]
    closed_promise: Rc<Promise>,
}

impl ReadableStreamDefaultReader {
    /// <https://streams.spec.whatwg.org/#default-reader-constructor>
    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<SafeHandleObject>,
        stream: DomRoot<ReadableStream>,
    ) -> Fallible<DomRoot<Self>> {
        Self::new_with_proto(global, proto, stream)
    }

    pub fn new(global: &GlobalScope, stream: DomRoot<ReadableStream>) -> Fallible<DomRoot<Self>> {
        Self::new_with_proto(global, None, stream)
    }

    #[allow(crown::unrooted_must_root)]
    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<SafeHandleObject>,
        stream: DomRoot<ReadableStream>,
    ) -> Fallible<DomRoot<Self>> {
        Ok(reflect_dom_object_with_proto(
            Box::new(Self::new_inherited(global, stream)?),
            global,
            proto,
        ))
    }

    /// <https://streams.spec.whatwg.org/#set-up-readable-stream-default-reader>
    fn new_inherited(
        global: &GlobalScope,
        stream: DomRoot<ReadableStream>,
    ) -> Fallible<ReadableStreamDefaultReader> {
        // Step 1
        if stream.is_locked() {
            return Err(Error::Type("Stream is locked".to_owned()));
        }

        // Step 2 & 3
        // <https://streams.spec.whatwg.org/#readable-stream-reader-generic-initialize>
        if stream.state() == StreamState::Readable {
            // Step 2.3
            Ok(ReadableStreamDefaultReader {
                reflector_: Reflector::new(),
                read_requests: DomRefCell::new(Vec::new()),
                stream: Some(stream.clone()),
                closed_promise: Promise::new(global),
            })
        } else if stream.state() == StreamState::Closed {
            // Step 2.4
            let cx = GlobalScope::get_cx();
            Ok(ReadableStreamDefaultReader {
                reflector_: Reflector::new(),
                read_requests: DomRefCell::new(Vec::new()),
                stream: Some(stream.clone()),
                closed_promise: Promise::new_resolved(global, cx, SafeHandleValue::undefined())?,
            })
        } else {
            // Step 2.5
            // Step 2.5.1
            assert_eq!(stream.state(), StreamState::Errored);

            // Step 2.5.2
            // TODO: Step 2.5.3 Set reader.[[closedPromise]].[[PromiseIsHandled]] to true.
            let cx = GlobalScope::get_cx();
            rooted!(in(*cx) let stored_error = stream.stored_error());
            Ok(ReadableStreamDefaultReader {
                reflector_: Reflector::new(),
                read_requests: DomRefCell::new(Vec::new()),
                stream: Some(stream.clone()),
                closed_promise: Promise::new_rejected(global, cx, stored_error.handle())?,
            })
        }
    }
}

impl ReadableStreamDefaultReader {
    pub fn read_requests(&'_ self) -> std::cell::Ref<'_, Vec<ReadRequest>> {
        self.read_requests.borrow()
    }
    pub fn set_read_requests(&self, read_requests: Vec<ReadRequest>) {
        *self.read_requests.borrow_mut() = read_requests;
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

#[derive(Clone, JSTraceable, MallocSizeOf)]
pub struct ReadRequest {
    // TODO: Algorithms
}
