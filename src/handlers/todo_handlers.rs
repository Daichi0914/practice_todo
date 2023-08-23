fn create_todo(todo: &str) -> &str {
    todo
}

#[derive()]
enum TodoStatus {
    Done,
    InProgress,
    Undone
}

struct Todo {
    id: i32,
    action: String,
    status: TodoStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを返す() {
        // then
        assert_eq!(create_todo("掃除をする"), "掃除をする")
    }
}
