use std::io::{BufRead, BufReader, Error};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
// use tokio::process::Command;

pub struct Wiremock {}

impl Wiremock {
    pub fn start_server(mapping_path: String, port: i32, tx: Sender<String>) -> Result<(), Error> {
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
            .expect("failed to execute wiremock-standalone");

        // let op = child_process.wait_with_output().unwrap();

        // read process output asynchronously
        tokio::spawn(async move {
            let pipe = child_process.stdout.unwrap();
            let mut output = BufReader::new(pipe);
            let mut line = String::new();

            loop {
                output.read_line(&mut line).unwrap();
                // println!("{}", line);
                // let _ = tx.send(line.clone());
                let _ = tx.send("now".to_string());
            }
        });

        Ok(())
    }
}
