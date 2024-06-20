use dom_struct::dom_struct;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::reflector::Reflector;
use crate::dom::readablestreamgenericreader::ReadRequest;

/// <https://streams.spec.whatwg.org/#default-reader-class>
#[dom_struct]
pub struct ReadableStreamDefaultReader {
    reflector_: Reflector,
    read_requests: DomRefCell<Vec<ReadRequest>>,
}

impl ReadableStreamDefaultReader {
    pub fn read_requests(&'_ self) -> std::cell::Ref<'_, Vec<ReadRequest>> {
        self.read_requests.borrow()
    }
}
