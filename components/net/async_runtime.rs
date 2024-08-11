/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::Mutex;

use lazy_static::lazy_static;
use servo_config::pref;
use tokio::runtime::{self, Runtime};

lazy_static! {
    pub static ref HANDLE: Mutex<Option<Runtime>> = Mutex::new(Some(
        runtime::Builder::new_multi_thread()
            .thread_name("AsyncRuntime")
            .worker_threads(pref!(network.async_runtime.threads) as usize)
            .enable_all()
            .build()
            .unwrap()
    ));
}
