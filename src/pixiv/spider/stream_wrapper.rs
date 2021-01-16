use futures::Stream;
use std::{
    collections::VecDeque,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use async_std::sync::Mutex;
pub struct StreamWrapper<'a, C, O, Fut: 'a + Future<Output = (O, C)>, F: Fn(C) -> Fut> {
    future_fn: F,
    future: Option<Pin<Box<Fut>>>,
    context: Option<C>,
    pd: PhantomData<&'a C>,
}

impl<'a, C, O, Fut: Future<Output = (O, C)>, F: Fn(C) -> Fut> StreamWrapper<'a, C, O, Fut, F> {
    pub fn new(ctx: C, f: F) -> StreamWrapper<'a, C, O, Fut, F> {
        StreamWrapper {
            future_fn: f,
            future: None,
            context: Some(ctx),
            pd: PhantomData,
        }
    }
}
impl<'a, C, O, Fut: Future<Output = (O, C)>, F: Fn(C) -> Fut> Unpin
    for StreamWrapper<'a, C, O, Fut, F>
{
}
impl<'a, C, O, Fut: Future<Output = (O, C)>, F: Fn(C) -> Fut> Stream
    for StreamWrapper<'a, C, O, Fut, F>
{
    type Item = O;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.future.is_none() {
            let ctx = self.context.take().unwrap();
            let future = (self.future_fn)(ctx);
            self.future = Some(Box::pin(future));
        }
        let mut future = self.future.take().unwrap();
        match future.as_mut().poll(cx) {
            Poll::Pending => {
                self.future = Some(future);
                Poll::Pending
            }
            Poll::Ready((x, c)) => {
                self.context = Some(c);
                Poll::Ready(Some(x))
            }
        }
    }
}

pub struct AsyncQueue<T> {
    inner: Mutex<VecDeque<T>>,
}

impl<T> AsyncQueue<T> {
    pub fn new() -> AsyncQueue<T> {
        AsyncQueue {
            inner: Mutex::new(VecDeque::new()),
        }
    }
    pub async fn push(&self, v: T) {
        self.inner.lock().await.push_back(v)
    }
    pub async fn pop(&self) -> Option<T> {
        let mut inner = self.inner.lock().await;
        inner.pop_front()
    }
    pub async fn size(&self) -> usize {
        self.inner.lock().await.len()
    }
    pub async fn push_all(&self, v: Vec<T>) {
        for item in v {
            self.inner.lock().await.push_back(item);
        }
    }
}

pub struct RunnerContext<T> {
    pub queue: std::sync::Arc<AsyncQueue<T>>,
    pub client: super::super::base::PixivClient,
}
