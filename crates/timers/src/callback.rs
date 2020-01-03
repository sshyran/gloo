//! Callback-style timer APIs.

use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast, JsValue};
use web_sys::{Window, WorkerGlobalScope};

macro_rules! global {
    ($global:ident, $fun:expr) => {{
        let global: JsValue = js_sys::global().into();

        if Reflect::has(&global, &String::from("Window").into()).unwrap_throw() {
            let $global: Window = global.into();
            $fun
        } else if Reflect::has(&global, &String::from("WorkerGlobalScope").into()).unwrap_throw() {
            let $global: WorkerGlobalScope = global.into();
            $fun
        } else {
            throw_val(global.into())
        }
    }};
}

/// A scheduled timeout.
///
/// See `Timeout::new` for scheduling new timeouts.
///
/// Once scheduled, you can either `cancel` so that it doesn't run or `forget`
/// it so that it is un-cancel-able.
#[derive(Debug)]
#[must_use = "timeouts cancel on drop; either call `forget` or `drop` explicitly"]
pub struct Timeout {
    id: Option<i32>,
    closure: Option<Closure<dyn FnMut()>>,
}

impl Drop for Timeout {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            global!(global, global.clear_timeout_with_handle(id));
        }
    }
}

impl Timeout {
    /// Schedule a timeout to invoke `callback` in `millis` milliseconds from
    /// now.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// let timeout = Timeout::new(1_000, move || {
    ///     // Do something...
    /// });
    /// ```
    pub fn new<F>(millis: u32, callback: F) -> Timeout
    where
        F: 'static + FnOnce(),
    {
        let closure = Closure::once(callback);

        let id = global!(
            global,
            global
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref::<js_sys::Function>(),
                    millis as i32,
                )
                .unwrap_throw()
        );

        Timeout {
            id: Some(id),
            closure: Some(closure),
        }
    }

    /// Make this timeout uncancel-able.
    ///
    /// Returns the identifier returned by the original `setTimeout` call, and
    /// therefore you can still cancel the timeout by calling `clearTimeout`
    /// directly (perhaps via `web_sys::clear_timeout_with_handle`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// // We definitely want to do stuff, and aren't going to ever cancel this
    /// // timeout.
    /// Timeout::new(1_000, || {
    ///     // Do stuff...
    /// }).forget();
    /// ```
    pub fn forget(mut self) -> i32 {
        let id = self.id.take().unwrap_throw();
        self.closure.take().unwrap_throw().forget();
        id
    }

    /// Cancel this timeout so that the callback is not invoked after the time
    /// is up.
    ///
    /// The scheduled callback is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// let timeout = Timeout::new(1_000, || {
    ///     // Do stuff...
    /// });
    ///
    /// // If actually we didn't want to set a timer, then cancel it.
    /// if nevermind() {
    ///     timeout.cancel();
    /// }
    /// # fn nevermind() -> bool { true }
    /// ```
    pub fn cancel(mut self) -> Closure<dyn FnMut()> {
        self.closure.take().unwrap_throw()
    }
}

/// A scheduled interval.
///
/// See `Interval::new` for scheduling new intervals.
///
/// Once scheduled, you can either `cancel` so that it ceases to fire or `forget`
/// it so that it is un-cancel-able.
#[derive(Debug)]
#[must_use = "intervals cancel on drop; either call `forget` or `drop` explicitly"]
pub struct Interval {
    id: Option<i32>,
    closure: Option<Closure<dyn FnMut()>>,
}

impl Drop for Interval {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            global!(global, global.clear_interval_with_handle(id));
        }
    }
}

impl Interval {
    /// Schedule an interval to invoke `callback` every `millis` milliseconds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// let interval = Interval::new(1_000, move || {
    ///     // Do something...
    /// });
    /// ```
    pub fn new<F>(millis: u32, callback: F) -> Interval
    where
        F: 'static + FnMut(),
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        let id = global!(
            global,
            global
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref::<js_sys::Function>(),
                    millis as i32,
                )
                .unwrap_throw()
        );

        Interval {
            id: Some(id),
            closure: Some(closure),
        }
    }

    /// Make this interval uncancel-able.
    ///
    /// Returns the identifier returned by the original `setInterval` call, and
    /// therefore you can still cancel the interval by calling `clearInterval`
    /// directly (perhaps via `web_sys::clear_interval_with_handle`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// // We want to do stuff every second, indefinitely.
    /// Interval::new(1_000, || {
    ///     // Do stuff...
    /// }).forget();
    /// ```
    pub fn forget(mut self) -> i32 {
        let id = self.id.take().unwrap_throw();
        self.closure.take().unwrap_throw().forget();
        id
    }

    /// Cancel this interval so that the callback is no longer periodically
    /// invoked.
    ///
    /// The scheduled callback is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// let interval = Interval::new(1_000, || {
    ///     // Do stuff...
    /// });
    ///
    /// // If we don't want this interval to run anymore, then cancel it.
    /// if nevermind() {
    ///     interval.cancel();
    /// }
    /// # fn nevermind() -> bool { true }
    /// ```
    pub fn cancel(mut self) -> Closure<dyn FnMut()> {
        self.closure.take().unwrap_throw()
    }
}
