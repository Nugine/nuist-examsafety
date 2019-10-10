import requests
import time

try:
    with open("account.txt") as f:
        xuehao, password = f.read().split(' ')
except Exception:
    xuehao = input('xuehao: ')
    password = input('password: ')

sess = requests.session()

form = {
    "xuehao": xuehao,
    "password": password,
    "postflag": '1',
    "cmd": "login",
    "role": '0'
}

LOGIN_URL = "http://examsafety.nuist.edu.cn/exam_login.php"
resp = sess.post(LOGIN_URL, data=form)
assert(resp.status_code == 200)
print(resp.cookies['wsess'])

HB_URL = "http://examsafety.nuist.edu.cn/exam_xuexi_online.php"

while True:
    resp = sess.post(HB_URL, data={"cmd": "xuexi_online"})
    print(resp.json()['shichang'])
    time.sleep(59)
