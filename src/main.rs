use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::timer::delay_for;

struct Account {
    xuehao: String,
    cookie: String,
    count: usize,
}

fn load_accounts(path: &str) -> std::io::Result<Vec<Account>> {
    let file = File::open(path)?;
    let mut accounts = vec![];
    for line in BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(" ").collect();
        assert!(parts.len() == 3);
        accounts.push(Account {
            xuehao: parts[0].to_owned(),
            cookie: parts[1].to_owned(),
            count: parts[2].parse().expect("invalid count"),
        });
    }
    Ok(accounts)
}

async fn exam(account: Account) -> reqwest::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();

    let cookie = format!("wsess={}", account.cookie);

    for i in 0..=account.count {
        const HB_URL: &'static str = "http://examsafety.nuist.edu.cn/exam_xuexi_online.php";
        let params = [("cmd", "xuexi_online")];

        let resp = client
            .post(HB_URL)
            .header("Cookie", &*cookie)
            .form(&params)
            .send()
            .await?;
        let value: Value = resp.json::<Value>().await?;

        println!(
            "[{}]: {}, {}/{}",
            account.xuehao, value["shichang"], i, account.count
        );

        delay_for(Duration::from_secs(59)).await;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new().unwrap();

    const PATH: &'static str = "accounts.txt";
    let accounts = load_accounts(PATH)?;

    for account in accounts {
        runtime.spawn(async move {
            let xuehao = account.xuehao.clone();
            let t0 = std::time::Instant::now();
            if let Err(e) = exam(account).await {
                dbg!(e);
            }
            println!("[{}]: stopped after {:?}", xuehao, t0.elapsed());
        });
    }

    runtime.shutdown_on_idle();

    Ok(())
}
