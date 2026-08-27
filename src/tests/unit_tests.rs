#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;
    use std::path::{PathBuf, Path};

    #[tokio::test]
    async fn test_create_item_dir() {
        let name = "test_dir";
        // Clean up before the test (if it already exists)
        if fs::try_exists(name).await.unwrap_or(false) {
            fs::remove_dir_all(name).await.unwrap();
        }

        let result = Fms::create_item_dir(name).await;
        assert_eq!(result, StatusFM::OK);

        // Verify the directory was created
        assert!(fs::try_exists(name).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_item_dir_at() {
        let name = "sub_test";
        let base_path = PathBuf::from("base");
        if fs::try_exists(&base_path).await.unwrap_or(false) {
            fs::remove_dir_all(&base_path).await.unwrap();
        }

        // Create the parent directory
        fs::create_dir(&base_path).await.unwrap();

        let fms = Fms::new();
        let result = fms.create_item_dir_at(name, &base_path).await;
        assert_eq!(result, StatusFM::OK);

        let expected = base_path.join(name);
        assert!(fs::try_exists(expected).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_item_file() {
        let name = "test_file.txt";
        if fs::try_exists(name).await.unwrap_or(false) {
            fs::remove_file(name).await.unwrap();
        }

        let data = "Hello, world!";
        let result = Fms::create_item_file(name, data).await;
        assert_eq!(result, StatusFM::OK);

        // Read back and verify
        let contents = fs::read_to_string(name).await.unwrap();
        assert_eq!(contents, data);
    }

    #[tokio::test]
    async fn test_create_item_file_at() {
        let name = "file_in_sub";
        let base_path = PathBuf::from("sub");
        if fs::try_exists(&base_path).await.unwrap_or(false) {
            fs::remove_dir_all(&base_path).await.unwrap();
        }

        // Create the parent directory
        fs::create_dir(&base_path).await.unwrap();

        let fms = Fms::new();
        let data = "File content";
        let result = fms.create_item_file_at(name, &base_path, data).await;
        assert_eq!(result, StatusFM::OK);

        let expected = base_path.join(name);
        let contents = fs::read_to_string(expected).await.unwrap();
        assert_eq!(contents, data);
    }

    #[tokio::test]
    async fn test_update_item_file() {
        let name = "update_test.txt";
        if fs::try_exists(name).await.unwrap_or(false) {
            fs::remove_file(name).await.unwrap();
        }

        // Create initial file
        Fms::create_item_file(name, "initial").await.unwrap();

        let fms = Fms::new();
        let result = fms.update_item_file(name, "updated").await;
        assert_eq!(result, StatusFM::OK);

        let contents = fs::read_to_string(name).await.unwrap();
        assert_eq!(contents, "updated");
    }

    #[tokio::test]
    async fn test_update_item_file_at() {
        let name = "update_sub.txt";
        let base_path = PathBuf::from("sub_update");
        if fs::try_exists(&base_path).await.unwrap_or(false) {
            fs::remove_dir_all(&base_path).await.unwrap();
        }

        // Create the parent directory and initial file
        fs::create_dir(&base_path).await.unwrap();
        let expected = base_path.join(name);
        Fms::create_item_file_at("initial", &base_path, "initial").await.unwrap();

        let fms = Fms::new();
        let result = fms.update_item_file_at(name, &base_path, "updated").await;
        assert_eq!(result, StatusFM::OK);

        let contents = fs::read_to_string(expected).await.unwrap();
        assert_eq!(contents, "updated");
    }

    #[tokio::test]
    async fn test_file_check_exists() {
        let name = "check_test.txt";
        if fs::try_exists(name).await.unwrap_or(false) {
            fs::remove_file(name).await.unwrap();
        }

        // Create the file
        Fms::create_item_file(name, "data").await.unwrap();

        let fms = Fms::new();
        let result = fms.file_check(name).await;
        assert_eq!(result, StatusFM::OK);
    }

    #[tokio::test]
    async fn test_file_check_not_found() {
        let name = "missing.txt";
        if fs::try_exists(name).await.unwrap_or(false) {
            fs::remove_file(name).await.unwrap();
        }

        let fms = Fms::new();
        let result = fms.file_check(name).await;
        assert_eq!(result, StatusFM::NOTFOUND);
    }
}
