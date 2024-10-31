use sizhe_chen_individual_project_2::{extract, transform_load, query};

#[test]
fn test_transform_load() {
    // Ensure the CSV is available by extracting it
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/candy-power-ranking/candy-data.csv";
    let file_path = "data/candy-data.csv";
    let directory = "data";
    
    extract(url, file_path, directory).expect("Failed to extract data");

    // Run the transform_load function
    let result = transform_load(file_path);

    // Check if the result is Ok and that the database connection is created successfully
    assert!(result.is_ok());
}

#[test]
fn test_query_select() {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/candy-power-ranking/candy-data.csv";
    let file_path = "data/candy-data.csv";
    let directory = "data";

    // Extract data and load it into the database
    extract(url, file_path, directory).expect("Failed to extract data");
    let conn = transform_load(file_path).expect("Failed to load data");

    // Define a simple SELECT query to verify data loading
    let select_query = "SELECT * FROM CandyData WHERE chocolate = 1 LIMIT 1;";
    let result = query(&conn, select_query);

    // Check if the query was executed successfully
    assert!(result.is_ok());
}

#[test]
fn test_query_insert_update_delete() {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/candy-power-ranking/candy-data.csv";
    let file_path = "data/candy-data.csv";
    let directory = "data";

    // Extract data and load it into the database
    extract(url, file_path, directory).expect("Failed to extract data");
    let conn = transform_load(file_path).expect("Failed to load data");

    // Test INSERT
    let insert_query = "INSERT INTO CandyData (competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus, sugarpercent, pricepercent, winpercent) VALUES ('Test Candy', 1, 0, 0, 0, 0, 1, 0, 1, 0, 0.75, 0.5, 60.0);";
    let insert_result = query(&conn, insert_query);
    assert!(insert_result.is_ok(), "Failed to insert data");

    // Test SELECT to verify the insert
    let select_query = "SELECT * FROM CandyData WHERE competitorname = 'Test Candy';";
    let select_result = query(&conn, select_query);
    assert!(select_result.is_ok(), "Failed to select data");

    // Test UPDATE
    let update_query = "UPDATE CandyData SET winpercent = 80.0 WHERE competitorname = 'Test Candy';";
    let update_result = query(&conn, update_query);
    assert!(update_result.is_ok(), "Failed to update data");

    // Test DELETE
    let delete_query = "DELETE FROM CandyData WHERE competitorname = 'Test Candy';";
    let delete_result = query(&conn, delete_query);
    assert!(delete_result.is_ok(), "Failed to delete data");
}
