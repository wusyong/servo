/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::gc::MutableHandleValue;
use js::jsapi::{
    AutoRequireNoGC, HandleObject, HandleValue, Heap, IsReadableStream, JSContext, JSObject,
};
use js::jsval::{JSVal, ObjectValue, UndefinedValue};
use js::rust::{HandleObject as SafeHandleObject, HandleValue as SafeHandleValue, IntoHandle};

use crate::dom::bindings::codegen::Bindings::QueuingStrategyBinding::QueuingStrategySize;
use crate::dom::bindings::codegen::Bindings::ReadableStreamDefaultControllerBinding::ReadableStreamDefaultControllerMethods;
use crate::dom::bindings::codegen::Bindings::UnderlyingSourceBinding::{
    ReadableStreamController, UnderlyingSource,
};
use crate::dom::bindings::conversions::{ConversionBehavior, ConversionResult};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::import::module::{ExceptionHandling, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::settings_stack::{AutoEntryScript, AutoIncumbentScript};
use crate::dom::bindings::utils::get_dictionary_property;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::Promise;
use crate::dom::readablestream::{ReadableStream, UnderlyingSourceAlgorithmsBase};
use crate::js::conversions::FromJSValConvertible;
use crate::realms::{enter_realm, InRealm};
use crate::script_runtime::JSContext as SafeJSContext;

/// <https://streams.spec.whatwg.org/#rs-default-controller-class-definition>
#[dom_struct]
pub struct ReadableStreamDefaultController {
    reflector_: Reflector,
}

impl ReadableStreamDefaultController {
    fn new_inherited() -> Self {
        Self {
            reflector_: Reflector::new(),
        }
    }

    fn new(global: &GlobalScope) -> DomRoot<Self> {
        reflect_dom_object(Box::new(Self::new_inherited()), global)
    }
}

impl ReadableStreamDefaultControllerMethods for ReadableStreamDefaultController {
    fn GetDesiredSize(&self) -> Option<f64> {
        todo!()
    }

    fn Close(&self) -> Fallible<()> {
        todo!()
    }

    fn Enqueue(&self, cx: SafeJSContext, chunk: SafeHandleValue) -> Fallible<()> {
        todo!()
    }

    fn Error(&self, cx: SafeJSContext, e: SafeHandleValue) -> Fallible<()> {
        todo!()
    }
}

/// <https://streams.spec.whatwg.org/#set-up-readable-stream-default-controller-from-underlying-source>
pub fn setup_readable_stream_default_controller_from_underlying_source(
    cx: SafeJSContext,
    stream: DomRoot<ReadableStream>,
    underlying_source_obj: SafeHandleObject,
    underlying_source_dict: UnderlyingSource,
    highwatermark: f64,
    size_algorithm: Rc<QueuingStrategySize>,
) -> Fallible<()> {
    // Step 1.
    let controller = ReadableStreamDefaultController::new(&*stream.global());

    // Step 2. - 7. See UnderlyingSourceAlgorithms
    let algorithms = UnderlyingSourceAlgorithms::new(underlying_source_dict, underlying_source_obj);

    todo!()
}

/// <https://streams.spec.whatwg.org/#set-up-readable-stream-default-controller>
fn set_up_readable_stream_default_controller() {}

#[derive(JSTraceable, MallocSizeOf)]
pub struct UnderlyingSourceAlgorithms {
    #[ignore_malloc_size_of = "bindings from mozjs"]
    underlying_source_dict: UnderlyingSource,
    #[ignore_malloc_size_of = "mozjs"]
    underlying_source_obj: Heap<*mut JSObject>,
}

impl UnderlyingSourceAlgorithms {
    pub fn new(underlying_source_dict: UnderlyingSource, obj: SafeHandleObject) -> Self {
        let underlying_source_obj = Heap::default();
        underlying_source_obj.set(obj.get());
        Self {
            underlying_source_dict,
            underlying_source_obj,
        }
    }
}

impl UnderlyingSourceAlgorithms {
    fn start(
        &self,
        cx: SafeJSContext,
        controller: ReadableStreamController,
        mut retval: MutableHandleValue,
    ) -> Fallible<()> {
        // Step 2
        rooted!(in(*cx) let mut val = UndefinedValue());
        // Step 5
        if let Some(callback) = &self.underlying_source_dict.start {
            // val.set(callback.Call_(
            //     &self.underlying_source_obj,
            //     controller,
            //     ExceptionHandling::Rethrow,
            // )?);
        }

        retval.set(val.get());
        Ok(())
    }

    fn pull(
        &self,
        cx: SafeJSContext,
        controller: ReadableStreamController,
    ) -> Fallible<Rc<Promise>> {
        todo!()
    }

    fn cancel(&self, cx: SafeJSContext, reason: Option<HandleValue>) -> Fallible<Rc<Promise>> {
        todo!()
    }
}
