use serde::Serialize;
use crate::models::todo_models::Todo;

fn create_todo(todo: Todo) -> Todo {
    todo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを返す() {
        // given
        let todo = Todo::new(String::from("掃除をする"));

        // when
        let result = create_todo(todo.clone());

        // then
        assert_eq!(result, todo)
    }
}
