use web_sys::{window, ImageData, HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d, Blob};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures::channel::oneshot::{channel, Receiver, Sender};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

//Use like:
//let blob = CanvasToBlobFuture::new(canvas).await;

pub struct CanvasToBlobFuture {
    pub canvas: HtmlCanvasElement,
    state: CanvasToBlobState,
    closure: Option<Closure<dyn FnMut(Blob)>>,
}

impl CanvasToBlobFuture {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self {
            canvas,
            state: CanvasToBlobState::Empty,
            closure: None
        }
    }
}

enum CanvasToBlobState {
    Empty,
    Loading {
        receiver: Receiver<(Blob)>,
    },
}

impl Future for CanvasToBlobFuture {
    type Output = Blob;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match &mut self.state {
            CanvasToBlobState::Empty => {
                //success callback
                let waker = ctx.waker().clone();
                let (sender, receiver): (Sender<Blob>, Receiver<Blob>) = channel();
                let mut sender = Option::from(sender);
                let closure = Closure::wrap(Box::new(move |blob| {
                    sender.take().unwrap_throw().send(blob).unwrap_throw();
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(Blob)>);

                self.canvas.to_blob(closure.as_ref().unchecked_ref());

                self.state = CanvasToBlobState::Loading {
                    receiver,
                };
                self.closure = Some(closure);

                //notify the task that we're now loading
                ctx.waker().wake_by_ref();

                Poll::Pending
            }

            CanvasToBlobState::Loading { receiver } => {
                //if let Poll::Ready(value) = Receiver::poll(Pin::new(receiver_err), ctx) {

                let mut is_cancelled = false;

                let state = match receiver.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };

                if let Some(blob) = state {
                    Poll::Ready(blob.clone())
                } else {
                    if !is_cancelled {
                        //ctx.waker().wake_by_ref();
                    }
                    Poll::Pending
                }
            }
        }
    }
}
