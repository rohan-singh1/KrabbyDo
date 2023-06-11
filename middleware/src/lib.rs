use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use tokio_stream::StreamExt as TokioStreamExt;

/// Struct to store event data
#[derive(Debug, Clone, PartialEq)]
pub struct EventEntry {
    pub unique_id: String,
    pub title: String,
    pub details: String,
    pub date_time: DateTime<Utc>,
    pub is_done: bool,
}

impl EventEntry {
    pub fn new(
        unique_id: String,
        title: String,
        details: String,
        date_time: DateTime<Utc>,
        is_done: bool,
    ) -> Self {
        EventEntry {
            unique_id,
            title,
            details,
            date_time,
            is_done,
        }
    }

    pub async fn add_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Event added to MongoDB");
        let client = create_mongodb_client().await?;
        // Get a handle to the "todos" collection in the "tasks" database
        let db = client.database("events");
        let collection = db.collection("todos");

        // Create a document representing the ToDo task
        let document = doc! {
            "title": self.title.clone(),
            "details": self.details.clone(),
            "date_time": self.date_time.to_rfc3339(),
            "is_done":true,
        };

        // Insert the document into the collection
        collection.insert_one(document, None).await?;

        Ok(())
    }

    pub fn update_task(&self) {
        println!("Event Updated !")
    }

    pub fn delete_or_mark_completed(&self) {
        println!("Event Completed !")
    }

    pub async fn get_all_tasks() -> Result<Vec<EventEntry>, Box<dyn std::error::Error>> {
        let client = create_mongodb_client().await?;
        let db = client.database("events");
        let collection: Collection<Document> = db.collection("todos");

        // Find all documents in the collection
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the cursor using the `try_next` method
        let mut tasks = Vec::new();

        while let Some(result) = TokioStreamExt::try_next(&mut cursor).await? {
            // Extract task data from the document
            let unique_id = result.get_object_id("_id")?.to_hex();
            let title = result.get_str("title")?.to_string();
            let details = result.get_str("details")?.to_string();
            let date_time_str = result.get_str("date_time")?;
            let date_time = DateTime::parse_from_rfc3339(date_time_str)?.with_timezone(&Utc);
            let is_done = result.get_bool("is_done")?;

            // Create a new EventEntry instance
            let task = EventEntry::new(unique_id, title, details, date_time, is_done);

            // Add the task to the vector
            tasks.push(task);
        }
        println!("Tasks: {:?}", tasks);
        Ok(tasks)
    }
}

pub async fn create_mongodb_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[test]
fn test_add_task() {
    let task_name = String::from("KrabbyDo new setup 3");
    let task_desc = String::from("First input Done! Mongo Setup successful3");
    let reminder_time = Utc::now();
    let is_completed = false;

    let event_entry = EventEntry::new(
        String::from(""),
        task_name,
        task_desc,
        reminder_time,
        is_completed,
    );
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Run the add_task function asynchronously
    let result = rt.block_on(async { event_entry.add_event().await });

    // Assert that the add_task function succeeded
    assert!(result.is_ok(), "add_event failed");
}
#[test]

fn test_get_all_tasks() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Run the get_all_tasks function asynchronously
    let result = rt.block_on(async { EventEntry::get_all_tasks().await });

    // Print the error message if the test fails
    if let Err(ref err) = result {
        println!("Error: {}", err);
    }

    // Assert that the get_all_tasks function succeeded
    assert!(result.is_ok(), "get_all_tasks failed");
}
