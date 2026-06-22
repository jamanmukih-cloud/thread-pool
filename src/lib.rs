pub enum Priority { High, Normal, Low }

pub struct ThreadPool { size: usize }

impl ThreadPool {
    pub fn new(size: usize) -> Self { Self { size } }
    
    pub fn submit<F>(&self, _priority: Priority, _task: F)
    where F: FnOnce() + Send + 'static {
    }
    
    pub fn shutdown(&self) {}
}
