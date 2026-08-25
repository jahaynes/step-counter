use std::sync::Arc;

pub struct AppState<S> {
    pub step_counter: Arc<S>,
}

impl<S> Clone for AppState<S> {
    fn clone(&self) -> Self {
        Self {
            step_counter: Arc::clone(&self.step_counter),
        }
    }
}

pub mod api;
pub mod web;
