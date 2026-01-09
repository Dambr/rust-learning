use std::thread;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn({
        let data_clone = data.clone();
        move || {
            for i in data_clone {
                println!("Thread: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    for i in data {
        println!("Main Thread: {}", i);
        thread::sleep(Duration::from_secs(2));
    }

    handle.join().unwrap();

    let (task1, task2) = tokio::join!(
        simulate_download("File 1", 3),
        simulate_download("File 2", 2)
    );
    println!("{} and {} are downloaded", task1, task2);
}

async fn simulate_download(file_name: &str, seconds: u64) -> String {
    println!("Starting to download {}", file_name);
    sleep(Duration::from_secs(seconds)).await;
    println!("Finished to download {}", file_name);
    file_name.to_string()
}
