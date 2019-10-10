use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::timer::delay_for;

fn load_accounts(path: &str) -> std::io::Result<Vec<(String, String)>> {
    let file = File::open(path)?;
    let mut cookies = vec![];
    for line in BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(" ").collect();
        assert!(parts.len() == 2);
        cookies.push((parts[0].to_owned(), parts[1].to_owned()));
    }
    Ok(cookies)
}

async fn exam((xuehao, cookie): (String, String)) -> reqwest::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();

    const N: usize = 5 * 60;
    let cookie = format!("wsess={}", cookie);

    for _ in 0..N {
        const HB_URL: &'static str = "http://examsafety.nuist.edu.cn/exam_xuexi_online.php";
        let params = [("cmd", "xuexi_online")];

        let resp = client
            .post(HB_URL)
            .header("Cookie", &*cookie)
            .form(&params)
            .send()
            .await?;
        let value: Value = resp.json::<Value>().await?;
        println!("[{}]: {}", xuehao, value["shichang"]);
        delay_for(Duration::from_secs(60)).await;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new().unwrap();

    const PATH: &'static str = "accounts.txt";
    let accounts = load_accounts(PATH)?;

    for account in accounts {
        runtime.spawn(async move {
            if let Err(e) = exam(account).await {
                dbg!(e);
            }
            dbg!(std::time::Instant::now());
        });
    }

    runtime.shutdown_on_idle();

    Ok(())
}
