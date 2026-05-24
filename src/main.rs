use interprocess::local_socket::{
    GenericNamespaced,
    tokio::prelude::*,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::env;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let action = env::args()
        .nth(1)
        .expect("Usage: client <action>\nExample: client claude");


    let mut stdin_input = String::new();
    io::stdin().read_to_string(&mut stdin_input)?;


    let payload: serde_json::Value = serde_json::from_str(&stdin_input)
        .expect("stdin must be valid JSON");


    let message = serde_json::json!({
        "action": action,
        "payload": payload,
    });


    let name = "灯灯侑侑天下第一".to_ns_name::<GenericNamespaced>()?;
    let conn = LocalSocketStream::connect(name).await?;
    let mut conn = BufReader::new(conn);


    let mut json_str = serde_json::to_string(&message)?;
    json_str.push('\n');
    conn.get_mut().write_all(json_str.as_bytes()).await?;


    let mut response_line = String::new();
    conn.read_line(&mut response_line).await?;

    let response: serde_json::Value = serde_json::from_str(&response_line)?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    //测试用的json目前是ai乱写的
    use super::*;
    use std::path::Path;
    use std::fs;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;
    use std::io::Write;

    #[test]
    fn test_client_with_default_json() {
        let json_path = Path::new("xxx.json");
        assert!(
            json_path.exists(),
            "xxx.json not found — place it in the same folder"
        );
        let mut server = Command::new("cargo")
            .args(["run", "--bin", "server"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start server");
        thread::sleep(Duration::from_millis(500));
        let json_content = fs::read_to_string(json_path)
            .expect("Failed to read x.json");
        let mut client = Command::new("cargo")
            .args(["run", "--bin", "client", "--", "claude"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start client");

        if let Some(stdin) = client.stdin.as_mut() {
            stdin.write_all(json_content.as_bytes()).unwrap();
        }
        let output = client.wait_with_output().expect("Client failed");
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("Client output:\n{}", stdout);
        server.kill().ok();
        let response: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Response should be valid JSON");

        assert_eq!(response["status"], "ok", "Expected status ok");
    }
}