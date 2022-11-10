videodir-rust
=============

* [Building a REST and Web Socket API with Actix v3 and Rust](https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html)

Версия `videodir` на Rust. Оригинальная версия продолжает
падать (скорее всего при сканировании).

Сервис обеспечивает доступ к каталогу с видеофайлами на видеорегистраторе.
Используется для архивирования по сети.

Сервис работает с каталогами и файлами структура каталогов для разных регистраторов своя.
Для сервиса это не важно. Через REST API клиент работает
с каталогами и отдельными файлами.

config
------

videodir.conf - TOML format

    LogLevel = "info"
    # ServerAddr = ":8443"
    
    # HTTPS data``
    Cert = "localhost.crt"
    Key = "localhost.key"
    
    # array pathes for video data directories
    VideoDirs = [ "./video1/", "./video2/" ]

В windows нужно удваивать обратный слэш для VideoDirs.
Также двойной слэш возвращается и в результатах запросов с windows
сервера.

Handlers
--------

| Handlers         | Query Type | Result                                                                                                                   |
|------------------|------------|--------------------------------------------------------------------------------------------------------------------------|
| /                | GET        | return index.html no auth                                                                                                |
| /login           | POST       | post {"username: "some", "password": "pass"} return {"token": "JWT TOKEN"}                                               |
| /api/v1/version  | GET        | return {"version": "0.1"}                                                                                                |
| /api/v1/volumes  | GET        | get array volumes with video dirs                                                                                        |
| /api/v1/list     | POST       | post { "path": [ "/24-01-18 01/" ] } get directory list, scan all volumes, path may be empty for root directory          |
| /api/v1/file     | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } get file, scan all volumes and return file stream, path not may be empty   |
| /api/v1/filesize | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } get filesize, scan all volumes and return file size                        |
| /api/v1/remove   | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } remove path (directory o single file)                                      |

1. remove return {"result": "OK"} or {"result": err.Error()},<br/> search path for remove on all volumes
2. path передаем как массив элементов пути, в противном случае, когда
   передаем путь из windows система видит ескейп последовательности
   вместо путей.

security
--------

Use HTTPS и JWT token (SigningMethodHS384)

Для HTTPS использовал RSA ключи, эти же ключи использовал для
подписи и проверки JWT. RSA используется в JWT библиотеке,
менять ничего не хотелось.

    openssl req \
        -x509 \
        -nodes \
        -newkey rsa:2048 \
        -keyout server.key \
        -out server.crt \
        -days 3650 \
        -subj "/C=RU/ST=SanktPetersburg/L=SanktPetersburg/O=Security/OU=IT Department/CN=*"

Использовал сгенерированные на основании rost.cert RSA ключи.

Пароли храним в htpasswd - точку перед именем очень не любит Windows.

    # create htpasswd with bcrypt hashes
    htpasswd -cbB htpasswd admin admin
    # add or update bcrypt hash
    htpasswd -bB htpasswd dima dima

CLI для работы с htpasswd. Для работы достаточно htpasswd нулевого размера.

```bash
>videodir-rust -h

Description:
    videodir-rust tool

Sub-commands:
    videodir-rust list     list users from htpasswd
    videodir-rust add      add or update user in the htpasswd
    videodir-rust remove   remove user from htpasswd

>videodir-rust add --help
videodir-rust add <name> <password>

Description:
    add or update user in the htpasswd

Arguments:
    name       user name
    password   password
    
>videodir-rust remove --help
 videodir-rust remove <name>
 
 Description:
     remove user from htpasswd
 
 Arguments:
     name   user name
```

Windows service
---------------

Словил проблему с инициализацией приложения. Для правильного конфигурирования
логгера нужно, чтобы приложение было создано в режиме сервиса (nonInteractive mode).
Поэтому приложение нельзя инициализировать сразу - есть два режима
запуска из командной строки и в режиме сервиса. Режим сервиса тоже два варианта
старт с ключом `start` или из апплета Сервис. 

Used crates
-----------

1. [bindata](https://github.com/glassbearInc/rs-bindata)  is a rust macro which loads files into your rust binary at compile time.
2. htpasswd based on [htpasswd-verify](https://github.com/aQaTL/htpasswd-verify) and
   [passivized_htpasswd](https://github.com/iamjpotts/passivized_htpasswd)
3. [toml](https://github.com/toml-rs/toml/tree/main/crates/toml) A TOML-parsing library


cross compilation
-----------------

```bash
# windows service
make service
# linux binary
make linux
```
