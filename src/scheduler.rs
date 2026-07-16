use crate::task::Task;

pub struct Scheduler {
    tasks: Vec<Box<dyn Task>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn add_task<T: Task + 'static>(&mut self, task: T) {
        self.tasks.push(Box::new(task));
    }

    pub fn update(&mut self) {
        for task in self.tasks.iter_mut() {
            task.update();
        }
    }
}