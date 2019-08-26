# Logscroll [![Build Status](https://travis-ci.org/meain/logscroll.svg?branch=master)](https://travis-ci.org/meain/logscroll)

> Provide a limited window for logs


<p align="center">
  <img src="https://i.imgur.com/UxXH6cU.gif">
</p>

### Installation

You can install it using cargo:

```
cargo install logscroll
```


### Usage

You can just pipe any log to `logscroll`. It also accepts two optional args `<height>` and `<width>`

```
log_generating_command | logscroll <height> <width>
```

*When it is not running using a tty, it just forewords the logs.*


