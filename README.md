# Tola

## Introduction

`Tola`: A static site generator for Typst-based blog.  

Note: this tool will not provide any Typst template, you should write your own Typst code for html side.  
`Tola` merely acts as a glue layer, handling the most tedious tasks unrelated to Typst itself.  

e.g.,  
- watch changes in multiple files and recompile  
- provide a local server to preview  
- to prevent users from typing tedious/repetitive command options, like `typst compile --features html --format html --root ./  --font-path ./ xxx.typ xxx/index.html`  

Please focus on Typst code itself(with `Tola`!)  

## Installation

1. typing `cargo install tola` to get `Tola`
2. install the binary in [release](https://github.com/KawaYww/tola/releases) page


## Usage

- ...

```text
A static site generator for typst-based blog

Usage: tola [OPTIONS] [COMMAND]

Commands:
  serve  Serve the site. Rebuild and reload on change automatically
  built  Deletes the output directory if there is one and rebuilds the site
  help   Print this message or the help of the given subcommand(s)

Options:
  -o, --output-dir <OUTPUT_DIR>    Output directory path [default: public]
  -c, --content-dir <CONTENT_DIR>  Content directory path [default: content]
  -a, --assets-dir <ASSETS_DIR>    Assets directory path [default: assets]
  -h, --help                       Print help
  -V, --version                    Print version
```

You should keep the directory structure identical to the below:

```text
.
├── assets
│   ├── fonts
│   ├── iconfonts
│   ├── images
│   ├── scripts
│   ├── styles
├── content
│   ├── posts/
│   ├── categories/
│   ├── home.typ
│   ├── programming.typ
├── templates
│   └── normal.typ
└── utils
    └── main.typ
```

Files under the `content/` directory are mapped to their respective routes:  
e.g., `content/posts/examples/aaa.typ` -> `https://127.0.0.1:8080/posts/examples/aaa`  
(`home.typ` will be specially included in `index.html`)  

```text
http://127.0.0.1:8000:
├── assets
│   ├── fonts
│   ├── iconfonts
│   ├── images
│   ├── scripts
│   └─ styles
├── posts/
├── categories/
└── index.html
```


