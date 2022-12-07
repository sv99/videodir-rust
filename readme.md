videodir-rust
=============

* [Building a REST and Web Socket API with Actix v3 and Rust](https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html)
* [How to pass reference to this static lifetime struct to each worker stared by the server?](https://stackoverflow.com/questions/73307880/how-to-pass-reference-to-this-static-lifetime-struct-to-each-worker-stared-by-th)
* [Building REST APIs in Rust with Actix Web](https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/)

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

| Handlers         | Query Type | Result                                                                                                                                                              |
|------------------|------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| /                | GET        | return index.html no auth                                                                                                                                           |
| /login           | POST       | post {"username: "some", "password": "pass"} <br/> return { "token": \<bearer JWT TOKEN\> }                                                                         |
| /api/v1/version  | GET        | return {"version": "0.1"}                                                                                                                                           |
| /api/v1/volumes  | GET        | return ["./videoItv/"] <br/> get array volumes with video dirs                                                                                                      |
| /api/v1/list     | POST       | post { "path": [ "/24-01-18 01/" ] } <br> return: [ "24-01-18 01", "25-01-18 02" ] <br/> get directory list, scan all volumes, path may be empty for root directory |
| /api/v1/file     | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } <br/> get file, scan all volumes and return file stream, path not may be empty                                        |
| /api/v1/filesize | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } <br/> return: { "size": 8 } <br/> get filesize, scan all volumes and return file size                                 |
| /api/v1/remove   | POST       | post { "path": [ "/24-01-18 01/", "0._02" ] } <br/> return {"result": "OK"} or {"result": err.Error()} <br/> remove path (directory o single file)                  |

1. path передаем как массив элементов пути, в противном случае, когда
   передаем путь из windows система видит ескейп последовательности
   вместо путей.

security
--------

* [Actix-Web Basic And Bearer Authentication Examples](https://turreta.com/2020/06/07/actix-web-basic-and-bearer-authentication-examples/)
* [How to use JWT with Rust](https://tms-dev-blog.com/how-to-use-jwt-with-rust-learn-the-basics/)
* [JWT realisation based on emreyalvac/actix-web-jwt](https://github.com/emreyalvac/actix-web-jwt)

Use HTTPS и Bearer JWT token (SigningMethodHS384) auth scheme.

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

На первом этапе возврат к запуску через оболочку [nssm](https://www.helpmegeek.com/run-applications-as-windows-service/)

Used crates
-----------

1. [bindata](https://github.com/glassbearInc/rs-bindata)  is a rust macro which loads files into your rust binary at compile time.
2. htpasswd based on [htpasswd-verify](https://github.com/aQaTL/htpasswd-verify) and
   [passivized_htpasswd](https://github.com/iamjpotts/passivized_htpasswd)
3. [toml](https://github.com/toml-rs/toml/tree/main/crates/toml) A TOML-parsing library
4. 


cross compilation
-----------------

.cargo single target - 485Mb

```bash
# cross compilation to windows
rustup target list
rustup target add x86_64-pc-windows-msvc
```

Простого решения как в Golang нет. По крайней мере нужно что-то делать с линкером и копировать библиотеки.
Проблема в компиляции нативного кода в библиотеках для поддержки криптографии.
Возможно, более простое решение компилировать сервис непосредственно на Windows?

todo
----

1. logging
2. custom error
3. windows service and cross compilation
