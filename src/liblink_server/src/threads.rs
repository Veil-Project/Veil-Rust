// Copyright 2020 Veil Rust Developers
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use crate::Result;
use num_cpus;
use std::sync;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Clone, Copy)]
pub struct WorkerSharedData {
    active_count: AtomicUsize,
    queue_count: AtomicUsize,
    panic_count: AtomicUsize,
}

impl WorkerSharedData {
    fn is_working(&self) -> bool {
        self.active_count.load(Ordering::SeqCst) > 0
    }

    fn has_work(&self) -> bool {
        self.queue_count.load(Ordering::SeqCst) > 0 || self.is_working()
    }
}

#[derive(Clone, Copy)]
pub struct WorkerConfig {
    name: String,
    stack_size: Option<usize>,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn spawn(
        id: usize,
        cfg: WorkerConfig,
        rx: sync::Arc<sync::Mutex<mpsc::Receiver<Message>>>,
        shared: sync::Arc<WorkerSharedData>,
    ) -> Result<Self> {
        let mut builder = thread::Builder::new().name(cfg.name);

        if let Some(size) = cfg.stack_size {
            builder = builder.stack_size(size);
        }

        let thread = builder.spawn(move || loop {
            let msg = rx
                .lock()
                .expect("Worker::spawn unable to get a lock on the receiver.")
                .recv()
                .unwrap();

            match msg {
                Message::NewJob(job) => {
                    shared.active_count.fetch_add(1, Ordering::SeqCst);
                    shared.queue_count.fetch_sub(1, Ordering::SeqCst);

                    job.call_box();

                    shared.active_count.fetch_sub(1, Ordering::SeqCst);
                }
                Message::Terminate => break,
            }
        })?;

        Ok(Worker {
            id: cfg.id,
            thread: Some(thread),
        })
    }
}

pub struct WorkerPoolConfig {
    pub name: String,
    pub size: Option<usize>,
    pub stack_size: Option<usize>,
}

pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    shared: sync::Arc<WorkerSharedData>,
}

impl WorkerPool {
    pub fn new(cfg: WorkerPoolConfig) -> Result<Self> {
        // TODO assert amount is lower than what get thread count returns.
        let size = cfg.size.unwrap_or_else(num_cpus::get());
        let (sender, receiver) = mpsc::channel();
        let receiver = sync::Arc::new(sync::Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        let shared = sync::Arc::new(WorkerSharedData {
            active_count: AtomicUsize::new(0),
            queue_count: AtomicUsize::new(0),
            panic_count: AtomicUsize::new(0),
        });
        let worker_cfg = WorkerConfig {
            name: cfg.name,
            stack_size: cfg.stack_size,
        };

        for id in 0..size {
            workers.push(Worker::spawn(
                id,
                worker_cfg.clone(),
                receiver.clone(),
                shared.clone(),
            )?);
        }

        Ok(Self {
            workers,
            sender,
            shared,
        })
    }

    pub fn execute<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        self.queue.fetch_add(1, Ordering::SeqCst);
        self.sender.send(Message::NewJob(Box::new(f)))?;
        Ok(())
    }

    pub fn active_count(&self) -> &AtomicUsize {
        &self.shared.active_count
    }

    pub fn queue_count(&self) -> &AtomicUsize {
        &self.shared.queue_count
    }

    pub fn panic_count(&self) -> &AtomicUsize {
        &self.shared.panic_count
    }

    pub fn is_working(&self) -> bool {
        self.shared.is_working()
    }

    pub fn has_work(&self) -> bool {
        self.shared.has_work()
    }

    pub fn shutdown(&mut self) -> Result<()> {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate)?;
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join()?;
            }
        }

        Ok(())
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}
