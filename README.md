![Python >= 3.8](https://img.shields.io/badge/python->=3.8-red.svg) [![](https://badgen.net/github/release/deedy5/html2text_rs)](https://github.com/deedy5/html2text_rs/releases) [![](https://badge.fury.io/py/html2text_rs.svg)](https://pypi.org/project/html2text_rs) [![Downloads](https://static.pepy.tech/badge/html2text_rs/week)](https://pepy.tech/project/html2text_rs) [![CI](https://github.com/deedy5/html2text_rs/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/deedy5/html2text_rs/actions/workflows/CI.yml)

# html2text_rs
Convert HTML to markdown or plain text.</br>
Python binding to the rust [rust-html2text](https://github.com/jugglerchris/rust-html2text) library.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [text_markdown()](#1-text_markdown)
  - [text_plain()](#2-text_plain)
  - [text_rich()](#3-text_rich)

## Installation

```python
pip install -U html2text_rs
```

## Usage
### 1. text_markdown()
```python
def text_markdown(html: str, width: int = 100):
    """Convert HTML to markdown text.

    Args:
        html (str): input html text.
        width (int): wrap text to width columns. Default is 100.

    """
```
example:
```python
import html2text_rs
import requests

resp = requests.get("https://en.wikipedia.org/wiki/AGM-88_HARM")

text_markdown = html2text_rs.text_markdown(resp.text)
print(text_markdown)
```
### 2. text_plain()
```python
def text_plain(html: str, width: int = 100):
    """Convert HTML to plain text.

    Args:
        html (str): input html text.
        width (int): wrap text to width columns. Default is 100.

    """
```
example:
```python
import html2text_rs
import requests

resp = requests.get("https://en.wikipedia.org/wiki/AGM-88_HARM")

text_plain = html2text_rs.text_plain(resp.text)
print(text_plain)
```
### 3. text_rich()
```python
def text_rich(html: str, width: int = 100):
    """Convert HTML to rich text.

    Args:
        html (str): input html text.
        width (int): wrap text to width columns. Default is 100.

    """
```
example:
```python
import html2text_rs
import requests

resp = requests.get("https://en.wikipedia.org/wiki/AGM-88_HARM")

text_rich = html2text_rs.text_rich(resp.text)
print(text_rich)
```
