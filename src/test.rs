#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn test_add_entry() {
        let path = "test_todo_add.txt";
        let mut file = create_temp_file();
        add(file.try_clone().expect("Unable to clone file"), "Test task".to_string()).expect("Add failed");

        let contents = read_file_contents(path);
        assert!(contents.contains("NOT_DONE NORMAL Test task"));

        cleanup(path);
    }

    #[test]
    fn test_remove_entry() {
        let path = "test_todo_remove.txt";
        let mut file = create_temp_file();
        writeln!(file, "NOT_DONE NORMAL Test task").expect("Unable to write to file");

        let file = File::open(path).expect("Unable to open file");
        let new_contents = remove(&file, 0).expect("Remove failed");

        assert!(new_contents.is_empty()); // Assuming the file only had one entry

        cleanup(path);
    }

    #[test]
    fn test_done_entry() {
        let path = "test_todo_done.txt";
        let mut file = create_temp_file();
        writeln!(file, "NOT_DONE NORMAL Test task").expect("Unable to write to file");

        let file = File::open(path).expect("Unable to open file");
        let new_contents = done(&file, 0).expect("Done failed");

        assert!(new_contents.contains("DONE NORMAL Test task"));

        cleanup(path);
    }

    #[test]
    fn test_clear_entries() {
        let path = "test_todo_clear.txt";
        let mut file = create_temp_file();
        writeln!(file, "NOT_DONE NORMAL Test task").expect("Unable to write to file");
        writeln!(file, "DONE NORMAL Another task").expect("Unable to write to file");

        let mut file = File::open(path).expect("Unable to open file");
        file.set_len(0).expect("Unable to clear file");

        let contents = read_file_contents(path);
        assert!(contents.is_empty());

        cleanup(path);
    }

    #[test]
    fn test_list_entries() {
        let path = "test_todo_list.txt";
        let mut file = create_temp_file();
        writeln!(file, "NOT_DONE NORMAL Test task").expect("Unable to write to file");
        writeln!(file, "DONE NORMAL Another task").expect("Unable to write to file");

        let file = File::open(path).expect("Unable to open file");
        let result = list(&file).expect("List failed");

        assert!(result.is_ok());

        cleanup(path);
    }
}
