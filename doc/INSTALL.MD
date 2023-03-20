# 如何安装
1. 首先在项目Release中下载最新的ZIP包(`publish.zip`)，解压到想要安装的文件夹
2. 安装PostgreSQL，这里以Debian为例(如果您希望使用MySQL请跳过此步看下面)

执行以下命令
```shell
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

sudo apt-get update

sudo apt-get -y install postgresql
```

然后使用postgres用户身份登录postgresql
```shell
sudo -u postgres psql
```

修改用户密码
```shell
\password
```

两次输入密码后输入`exit`即可推出psql管理工具，再输入一次`logout`以注销postgres用户身份。

然后推荐使用Navicat或者其他数据库管理工具连接，用户名为postgres，密码为上面设置的密码，端口为5432。

2. 安装MySQL

参见[安装教程](https://acytoo.com/ladder/install-mysql-on-debian11/)

3. 修改.env文件
修改zip压缩包中解压出来的.env文件，文件各个配置项目意义如下：
```text
LISTEN_ADDR = ""  # 监听地址，格式为IP:端口。如果只想监听本地则填写127.0.0.1:端口号，如果想直接接受远程链接填写0.0.0.0:端口号
DATABASE_URL = "postgres://username:password@host:port/database"  # 数据库连接URL，修改其中username、password、host、port和database为上面数据库中创建的信息
SESSION_KEY = "" # Session加密KEY，至少64位
SMTP_HOST="" # SMTP主机地址
SMTP_PORT="" # SMTP端口，一般为465，少数使用STARTTLS的提供商(例如Outlook)为587
SMTP_USERNAME="" # SMTP登录用户名，一般就是邮箱地址
SMTP_PASSWORD="" # SMTP密码
SMTP_ADDRESS="" # SMTP登录的邮箱地址
SMTP_NICKNAME="阳光小助手" # SMTP发件人昵称
SMTP_STARTTLS="false" # 是否使用STARTTLS协议登录SMTP，对部分使用STARTTLS协议的提供商(例如Outlook)需要设置为true
```

注意：如果您希望使用MySQL，请将DATABASE_URL中开头的`postgres://`改成`mysql://`

4. 运行
先给予运行权限
```shell
sudo chmod +x sunrun-rs
```

然后可以直接运行
```shell
./sunrun-rs
```

或者简单的放在后台运行
```shell
nohup ./sunrun-rs &
```