use std::collections::{vec_deque, VecDeque};

use super::{Customer, Declined, Enqueued};

pub struct CustomerQueue {
    customers: VecDeque<Customer<Enqueued>>,
    capacity: Option<usize>,
}

impl CustomerQueue {
    pub fn new() -> Self {
        Self {
            customers: VecDeque::new(),
            capacity: None,
        }
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = Some(capacity)
    }

    pub fn get_capacity(&self) -> Option<usize> {
        self.capacity
    }

    pub fn iter(&self) -> vec_deque::Iter<Customer<Enqueued>> {
        self.customers.iter()
    }

    pub fn len(&self) -> usize {
        self.customers.len()
    }

    pub fn free_positions(&self) -> Option<usize> {
        self.capacity.map(|cap| cap - self.len())
    }

    pub fn is_full(&self) -> bool {
        match self.capacity {
            None => false,
            Some(cap) => cap <= self.len(),
        }
    }

    pub fn enqueue(
        &mut self,
        mut incoming: Vec<Customer<Enqueued>>,
    ) -> Result<usize, Vec<Customer<Declined>>> {
        let mut enqueued: usize = 0;

        loop {
            if self.is_full() {
                return Err(incoming.into_iter().map(|c| c.decline()).collect());
            }
            match incoming.pop() {
                Some(customer) => {
                    self.customers.push_back(customer);
                    enqueued += 1;
                }
                None => {
                    return Ok(enqueued);
                }
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<Customer<Enqueued>> {
        self.customers.pop_front()
    }
}
