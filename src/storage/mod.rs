pub mod ingest;
pub mod note;
pub mod project;
pub mod todo;

use note::Note;
use project::Project;
use todo::Todo;

pub struct Storage {
    pub notes: Vec<Note>,
    pub todos: Vec<Todo>,
    pub projects: Vec<Project>,
}

impl Storage {
    pub fn load_or_init() -> Result<Self, std::io::Error> {
        Ok(Self {
            notes: Vec::new(),
            todos: Vec::new(),
            projects: Vec::new(),
        })
    }
}
