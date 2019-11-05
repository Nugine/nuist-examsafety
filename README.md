# nuist-examsafety

1. 最简方法

    打开挂机页面，在控制台里执行一句代码

    ```javascript
    confirm = () => true
    ```

2. python 实现

    [可选] 在项目目录下建立 account.txt，写入账号信息

    ```
    2018xxxxxxxx thisispassword
    ```

    执行 python 代码

    ```bash
    python3 heartbeat.py
    ```

3. Rust 实现

    在项目目录下建立 accounts.txt，写入学号、cookie 和发送次数

    ```
    2018xxxxxx01 thisiscookie1 60
    2018xxxxxx02 thisiscookie2 60
    2018xxxxxx03 thisiscookie3 60
    ```

    ```bash
    cargo run --release
    ```