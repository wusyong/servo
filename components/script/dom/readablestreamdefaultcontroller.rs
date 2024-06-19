/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::{Cell, OnceCell};
use std::collections::VecDeque;
use std::rc::Rc;

use dom_struct::dom_struct;
use js::gc::MutableHandleValue;
use js::jsapi::{HandleValue, Heap, JSObject};
use js::jsval::{JSVal, ObjectValue, UndefinedValue};
use js::rust::{HandleObject as SafeHandleObject, HandleValue as SafeHandleValue};

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::QueuingStrategyBinding::QueuingStrategySize;
use crate::dom::bindings::codegen::Bindings::ReadableStreamDefaultControllerBinding::ReadableStreamDefaultControllerMethods;
use crate::dom::bindings::codegen::Bindings::UnderlyingSourceBinding::{
    ReadableStreamController, UnderlyingSource,
};
use crate::dom::bindings::import::module::{ExceptionHandling, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::Promise;
use crate::dom::readablestream::ReadableStream;
use crate::script_runtime::JSContext as SafeJSContext;

/// <https://streams.spec.whatwg.org/#rs-default-controller-class-definition>
#[dom_struct]
pub struct ReadableStreamDefaultController {
    reflector_: Reflector,
    /// All algoritems packed together:
    /// - Close algorithm: A promise-returning algorithm, taking one argument (the cancel reason), which communicates a requested cancelation to the underlying source
    /// - Pull algorithm: A promise-returning algorithm that pulls data from the underlying source
    algorithms: UnderlyingSourceAlgorithms,
    /// A boolean flag indicating whether the stream has been closed by its underlying source, but still has chunks in its internal queue that have not yet been read
    close_requested: Cell<bool>,
    /// A boolean flag set to true if the stream’s mechanisms requested a call to the underlying source's pull algorithm to pull more data, but the pull could not yet be done since a previous call is still executing
    pull_again: Cell<bool>,
    /// A boolean flag set to true while the underlying source's pull algorithm is executing and the returned promise has not yet fulfilled, used to prevent reentrant calls
    pulling: Cell<bool>,
    /// A list representing the stream’s internal queue of chunks
    #[ignore_malloc_size_of = "Defined in mozjs"]
    queue: DomRefCell<VecDeque<Heap<JSVal>>>,
    /// A boolean flag indicating whether the underlying source has finished starting
    started: Cell<bool>,
    /// A number supplied to the constructor as part of the stream’s queuing strategy, indicating the point at which the stream will apply backpressure to its underlying source
    strategy_hwm: Cell<f64>,
    /// An algorithm to calculate the size of enqueued chunks, as part of the stream’s queuing strategy
    ///
    /// If missing use default value (1) per https://streams.spec.whatwg.org/#make-size-algorithm-from-size-function
    #[ignore_malloc_size_of = "Rc is hard"]
    strategy_size_algorithm: Rc<QueuingStrategySize>,
    // /// The ReadableStream instance controlled
    // stream: MutNullableDom<ReadableStream>,
}

impl ReadableStreamDefaultController {
    fn new_inherited() -> Self {
        Self {
            reflector_: Reflector::new(),
            queue: Default::default(),
            close_requested: Cell::new(false),
            pull_again: Cell::new(false),
            pulling: Cell::new(false),
            started: Cell::new(false),
            strategy_hwm: Cell::new(0.),
            strategy_size_algorithm: todo!(),
            algorithms: todo!(),
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

    // Step 8
    set_up_readable_stream_default_controller(
        cx,
        stream,
        controller,
        algorithms,
        highwatermark,
        size_algorithm,
    )
}

/// <https://streams.spec.whatwg.org/#set-up-readable-stream-default-controller>
fn set_up_readable_stream_default_controller(
    cx: SafeJSContext,
    stream: DomRoot<ReadableStream>,
    controller: DomRoot<ReadableStreamDefaultController>,
    algorithms: UnderlyingSourceAlgorithms,
    highwatermark: f64,
    size_algorithm: Rc<QueuingStrategySize>,
) -> Fallible<()> {
    // Step 1
    assert!(stream.controller().is_none());
    // Step 2
    stream.set_controller(ReadableStreamController::ReadableStreamDefaultController(
        controller.clone(),
    ));
    // Step 3 Perform ! ResetQueue(controller).
    controller.queue.borrow_mut().clear();
    // Step 4
    controller.started.set(false);
    controller.close_requested.set(false);
    controller.pull_again.set(false);
    controller.pulling.set(false);

    todo!()
}

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
            val.set(callback.call_with_existing_obj(
                &self.underlying_source_obj,
                controller,
                ExceptionHandling::Rethrow,
            )?);
        }

        retval.set(val.get());
        Ok(())
    }

    fn pull(
        &self,
        cx: SafeJSContext,
        controller: ReadableStreamController,
    ) -> Fallible<Rc<Promise>> {
        // Step 3 & 6
        if let Some(callback) = &self.underlying_source_dict.pull {
            callback.call_with_existing_obj(
                &self.underlying_source_obj,
                controller,
                ExceptionHandling::Rethrow,
            )
        } else {
            Promise::new_resolved(
                &GlobalScope::current().expect("No current global"),
                cx,
                SafeHandleValue::undefined(),
            )
        }
    }

    fn cancel(&self, cx: SafeJSContext, reason: Option<HandleValue>) -> Fallible<Rc<Promise>> {
        // Step 4 & 7
        if let Some(callback) = &self.underlying_source_dict.cancel {
            callback.call_with_existing_obj(
                &self.underlying_source_obj,
                reason,
                ExceptionHandling::Rethrow,
            )
        } else {
            Promise::new_resolved(
                &GlobalScope::current().expect("No current global"),
                cx,
                SafeHandleValue::undefined(),
            )
        }
    }
}
