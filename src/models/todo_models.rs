use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use serde::Serialize;

#[derive(PartialEq, Serialize, Debug, Clone)]
enum TodoStatus {
    Undone,
    InProgress,
    Done
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub struct Todo {
    id: u32,
    action: String,
    status: TodoStatus,
}

pub struct CreateTodo {
    action: String,
}

impl Todo {
    pub(crate) fn new(id: u32, action: String) -> Self {
        Todo {
            id,
            action,
            status: TodoStatus::Undone
        }
    }
}

pub trait TodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo;
}

type TodoDates = HashMap<u32, Todo>;

pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDates>>
}

impl TodoRepositoryForMemory {
    pub(crate) fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default()
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDates> {
        self.store.write().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as u32;
        let todo = Todo::new(id, payload.action.clone());
        store.insert(id, todo.clone());
        todo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを作成する() {
        // given
        let memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("掃除をする") };

        // when
        let create_todo = memory.create(payload);

        // then
        // TODO: insertしたstoreの中身をreadできるようにしたらテスト変更する
        assert_eq!(
            create_todo,
            Todo {
                id: 1,
                action: String::from("掃除をする"),
                status: TodoStatus::Undone,
            }
        );
    }

    #[test]
    fn todoを取得する() {
        // given
        let mut memory = TodoRepositoryForMemory::new();
        let payload = CreateTodo { action: String::from("勉強をする") };
        memory.create(payload);

        // when
        let todos = memory.read();

        // then
        assert_eq!(
            todos.get(1),
            Todo {
                id: 1,
                action: String::from("掃除をする"),
                status: TodoStatus::Undone,
            }
        )
    }
}
