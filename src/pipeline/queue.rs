use std::collections::VecDeque;
use crate::error::Result;
use crate::pipeline::engine::TranscodeEngine;
use crate::pipeline::job::TranscodeJob;
use crate::pipeline::progress::TranscodeProgress;

#[derive(Default)]
pub struct JobQueue {
    pub jobs: VecDeque<TranscodeJob>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
        }
    }

    pub fn push(&mut self, job: TranscodeJob) {
        self.jobs.push_back(job);
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    /// Process all jobs in the queue sequentially
    pub fn process_all<F>(&mut self, mut progress_callback: F) -> Result<usize>
    where
        F: FnMut(usize, usize, &TranscodeJob, &TranscodeProgress),
    {
        let total = self.jobs.len();
        let mut completed = 0;

        while let Some(job) = self.jobs.pop_front() {
            let job_idx = completed + 1;
            TranscodeEngine::run(&job, |prog| {
                progress_callback(job_idx, total, &job, prog);
            })?;
            completed += 1;
        }

        Ok(completed)
    }
}
