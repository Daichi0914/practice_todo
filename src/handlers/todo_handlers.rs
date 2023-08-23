

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todoを返す() {
        // then
        assert_eq!(create_todo("掃除をする"), "掃除をする")
    }
}
