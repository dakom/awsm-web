use std::pin::Pin;
use std::task::{Context, Poll};
use futures::stream::Stream;
use futures::channel::mpsc;
use wasm_bindgen::prelude::*;

pub struct ClosureStream {
    receiver: mpsc::UnboundedReceiver<JsValue>,
}

// get a tuple of a closure that can be passed to JS
// and an impl Stream of results
// the callback expects a _single_ JsValue in its arg
impl ClosureStream {
    pub fn new() -> (Closure<dyn FnMut(JsValue)>, Self) {
        let (sender, receiver) = mpsc::unbounded();

        let closure = Closure::new(move |data| {
            sender.unbounded_send(data).unwrap_throw();
        });

        (closure, Self { receiver })
    }
}

impl Stream for ClosureStream {
    type Item = JsValue;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}