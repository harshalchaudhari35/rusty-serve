Rusty Serve
======

**A http webserver written in rust that can serves static files.**

-------


###### Build and run

```shell
    $ cargo run
```

###### Access server in browser

http://127.0.0.1:8080/index.html


###### Python bindings and installation

```shell
    # setup python environment
    # create virtual env
    $ python -m venv my_env
    $ source my_env/bin/activate

    # install maturin which will help to create python native module
    $ pip install maturin

    # install in develop mode
    $ maturin develop

    # install full build whell using pip, add verbose flag to see what happens under the hood
    $ pip install . -v

    # run web server using python cmd script
    $ rustyserver
    ###### Access server in browser
    ######    http://127.0.0.1:8080/index.html

```

###### Further enhancements

- [ ] multithreaded request handling
- [X] convert to python native module