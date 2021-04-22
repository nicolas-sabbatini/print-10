# 10 Print
> 10 PRINT CHR$(205.5+RND(1)); : GOTO 10

A implementation of the program 10 print in [Rust](https://www.rust-lang.org/es) and using the crate [macroquad](https://github.com/not-fl3/macroquad).

![10-print](10-print.png)

The implementation tries to be the most faithful to the original source in respect of the aspect ratio and colors, if you resize the windows the proportion is going to get lost.

Something that cannot be reproduced with the approach is being a one-liner and be readable at the same time.

## Usage.

Simply execute:
```shell
cargo run
```

You can also build for web running:
```shell
cargo build --target wasm32-unknown-unknown --release
```
This will produce .wasm file in `target/release/wasm32-unknown-unknown/print-10.wasm`.

In the same folder of the `.wasam` file create the following .html to load it:

<details><summary>index.html</summary>

```html
<html lang="en">

<head>
    <meta charset="utf-8">
    <title>TITLE</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>

<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle_0.3.0.js"></script>
    <script>load("print-10.wasm");</script>
</body>

</html>
```

</details>

Finally, serve the html file and access it through your favorite browser.
