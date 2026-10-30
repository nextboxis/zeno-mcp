// src/dsa/ring_buffer.rs
// Memory-bounded Circular Ring Buffer for O(1) telemetry and log streaming.

use std::collections::VecDeque;

pub struct CircularRingBuffer<T> {
    capacity: usize,
    buffer: VecDeque<T>,
}

impl<T> CircularRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
