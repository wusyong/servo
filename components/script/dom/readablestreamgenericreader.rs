use std::rc::Rc;

use crate::dom::bindings::codegen::Bindings::FunctionBinding::Function;

#[derive(Clone, JSTraceable, MallocSizeOf)]
pub struct ReadRequest {
    #[ignore_malloc_size_of = "Rc"]
    chunk_steps: Rc<Function>,
    #[ignore_malloc_size_of = "Rc"]
    close_steps: Rc<Function>,
    #[ignore_malloc_size_of = "Rc"]
    error_steps: Rc<Function>,
}
