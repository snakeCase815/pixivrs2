use futures::Stream;
use std::{
    pin::Pin,
    future::Future,
    task::{Poll,Context}
};


pub struct FutureSelector<O> {
    futures : Vec<Pin<Box<dyn Future<Output = O>>>>,
}

impl<O> FutureSelector<O> {
    pub fn new() -> FutureSelector<O> {
        FutureSelector {
            futures : Vec::new()
        }
    }
    pub fn len(&self) -> usize {
        self.futures.len()
    }
    pub fn push(&mut self,future : Pin<Box<dyn Future<Output = O>>>) {
        self.futures.push(future);
    }
}

impl<O> Stream for FutureSelector<O> {
    type Item = O;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        for i in 0..self.futures.len() {
            match self.futures[i].as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(x) => {
                    self.futures.remove(i);
                    return Poll::Ready(Some(x))
                }
            }
        }
        Poll::Pending
    }
}

