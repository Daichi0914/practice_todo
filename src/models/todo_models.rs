use std::collections::HashMap;
use serde::Serialize;

#[derive(PartialEq, Serialize, Debug, Clone)]
enum TodoStatus {
    Done,
    InProgress,
    Undone
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub struct Todo {
    id: i32,
    action: String,
    status: TodoStatus,
}

pub struct CreateTodo {
    action: String,
}

impl Todo {
    pub(crate) fn new(action: String) -> Self {
        Todo {
            id: 1,
            action,
            status: TodoStatus::Undone
        }
    }
}

// TODO: メモリを実装する

pub trait TodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo;
}

type TodoDates = HashMap<i32, Todo>;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを作成する() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let todo = Todo::new(String::from("掃除をする"));

        // when
        memory::create(todo);

        // then
        assert_eq!(memory.get(&1), todo);
    }
}
