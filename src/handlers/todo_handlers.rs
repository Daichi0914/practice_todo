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

impl Todo {
    fn new(action: String, status: TodoStatus) -> Self {
        Todo {
            id: 1,
            action,
            status
        }
    }

    fn create_todo(todo: Todo) -> Todo {
        todo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを返す() {
        // given
        let todo = Todo::new(String::from("掃除をする"), TodoStatus::Undone);

        // when
        let result = Todo::create_todo(todo.clone());

        // then
        assert_eq!(result, todo)
    }
}
