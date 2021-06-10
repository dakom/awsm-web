// Thanks for the help Pauan!
use wasm_bindgen_futures::spawn_local;
use std::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
    rc::Rc,
    cell::RefCell
};
use futures::{
    future::{abortable, AbortHandle, select, Either},
    pin_mut
};
use gloo_timers::future::TimeoutFuture;

/// Simple way to spawn a future and cancel it by dropping the handle
pub fn spawn_handle<F, A>(fut: F) -> FutureHandle
where
    F: Future<Output = A> + 'static
{
    let (fut, handle) = abortable(fut);

    spawn_local(async move {
        fut.await;
    });

    FutureHandle { inner: handle }
}

pub struct FutureHandle {
    pub inner: AbortHandle 
}

impl Drop for FutureHandle {
    fn drop(&mut self) {
        self.inner.abort();
    }
}

/// makes it easier to run futures with a timeout
///
/// future_until(10_000, some_future).await
///
/// or
///
/// future_util(10_000, async move {
///    some_future1().await;
///    some_future2().await;
///    some_future3().await;
/// }).await
///
pub async fn future_until<F, A>(ms: u32, f: F) -> Option<A>
    where F: Future<Output = A> {

    pin_mut!(f);

    match select(f, TimeoutFuture::new(ms)).await {
        Either::Left((value, _)) => Some(value),
        Either::Right((_, _)) => None,
    }
}

/// Makes it easier to run a Future in the background with the ability to:
/// * cancel (explicitly or on Drop)
/// * swap it with a different Future
///
/// Stolen/Adapted with permission from Dominator (https://github.com/Pauan/rust-dominator/blob/24920fd7af3b1b782cb4e59ffe5986a5f7a9e083/examples/async/src/util.rs#L31)
///
/// Hold onto the AsyncLoader somewhere and call load(async move {...}) or cancel()
pub struct AsyncLoader {
    loading: Rc<RefCell<Option<AsyncState>>>,
}
impl Drop for AsyncLoader {
    fn drop(&mut self) {
        self.cancel();
    }
}

struct AsyncState {
    id: usize,
    handle: AbortHandle,
}

impl AsyncState {
    fn new(handle: AbortHandle) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);

        let id = ID.fetch_add(1, Ordering::SeqCst);

        Self { id, handle }
    }
}
impl AsyncLoader {
    pub fn new() -> Self {
        Self {
            loading: Rc::new(RefCell::new(None)),
        }
    }

    pub fn cancel(&self) {
        self.replace(None);
    }

    fn replace(&self, value: Option<AsyncState>) {
        let mut loading = self.loading.borrow_mut();

        if let Some(state) = loading.as_mut() {
            state.handle.abort();
        }

        *loading = value;
    }

    pub fn load<F>(&self, fut: F) where F: Future<Output = ()> + 'static {
        let (fut, handle) = abortable(fut);

        let state = AsyncState::new(handle);
        let id = state.id;

        self.replace(Some(state));

        let loading = self.loading.clone();

        spawn_local(async move {
            match fut.await {
                Ok(()) => {
                    let mut loading = loading.borrow_mut();

                    if let Some(current_id) = loading.as_ref().map(|x| x.id) {
                        // If it hasn't been overwritten with a new state...

                        if current_id == id {
                            *loading = None;
                        }
                    }
                },
                // It was already cancelled

                Err(_) => {},
            }
        });
    }

    pub fn is_loading(&self) -> bool {
        self.loading.borrow().is_some()
    }
}

