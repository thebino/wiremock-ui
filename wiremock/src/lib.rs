#[allow(unused_imports, unused_variables)]
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use notify::{RecursiveMode, Watcher};
use rand::random;

pub struct Wiremock {
}

#[derive(Debug)]
pub enum WiremockErrors {
    FileWatcherError,
    ThisDoesntLookGoodError,
    ImmaGetFiredError,
}

impl Wiremock {
    pub fn start_server(mapping_path: String, port: i32, tx: Sender<u32>) -> (Option<Child>, JoinHandle<String>) {
        // execute as a separate process
        let child_process = Command::new("java")
            .args([
                "-jar",
                "./wiremock-standalone-3.2.0.jar",
                "--root-dir",
                mapping_path.to_string().as_str(),
                "--port",
                format!("{}", port).as_str(),
                "--local-response-templating",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok();

        // read process output asynchronously
        let output_handler: JoinHandle<String> = thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build().unwrap();

            let _ = runtime.enter();

            runtime.block_on(async {
                // TODO: pipe output into `tx`
                // let pipe = child_process.unwrap().stdout.unwrap();
                // let mut output = BufReader::new(pipe);
                // let mut line = String::new();

                loop {
                    // output.read_line(&mut line).unwrap();
                    // println!("{}", line);

                    let next: u32 = random();
                    let _ = tx.send(next);
                    // wait 1 second
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            })
        });

        (child_process, output_handler)
    }

    pub fn stop_server(process: &mut Child, output: &mut JoinHandle<String>) {
        // kill output handler
        // let _ = output.join();

        // kill java process
        process.kill().unwrap();
    }

    pub fn start_filewatcher(path: &Path) -> Result<(), WiremockErrors> {
        let watcher = notify::recommended_watcher(|res| {
            match res {
                Ok(event) => {
                    println!("event: {:?}", event)
                    // TODO: invoke callback to reload mappings and scenarios
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        });

        match watcher {
            Ok(mut w) => {
                let result = w.watch(path, RecursiveMode::Recursive);

                Ok(result.unwrap())
            }
            Err(_) => {
                Err(WiremockErrors::FileWatcherError)
            }
        }
    }
}
