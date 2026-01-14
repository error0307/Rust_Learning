use std::sync::{mpsc};
use std::thread;
use std::time::{Duration};

#[derive(Debug)]
struct EventData {
    id: u32,
    description: String,
}

impl EventData {
    fn new(id :u32, description: &str)->EventData{
        EventData {
            id,
            description: String::from(description),
        }
    }
}

fn main(){
    // Initialize 
    let (tx, rx) = mpsc::channel::<EventData>(); // チャンネルは型を指定する
    let event_thread = thread::spawn(move || {
        for event in rx {
            println!("Received Event - ID: {}, Description: {}", event.id, event.description);
        }
    });

    // Send events
    for i in 1..6 {
        let event = EventData::new(i, &format!("Event number {}", i));
        tx.send(event).unwrap();
        println!("Sent Event ID: {}", i);
        thread::sleep(Duration::from_secs(1)); // Simulate some delay
    }
    drop(tx); // Close the channel
    event_thread.join().unwrap(); // Wait for the event thread to finish

}

