# Tauri w/ Axum

Hacking around "running" and Axum server inside Tauri. Anchor tag clicks create a primitive HTTP request which is sent to the Tauri backend. From there, it is passed to the axum server which provides a  response. The response body is passed back to the webview where it is set as the `document.documentElement.innerHTML`.  A lot more functionality could easily be built out like form submission, ajax/fetch support, etc.  This provides a simple pattern for writing a web app in rust and running it in a webview with Tauri.  I would speculate this is incredible performant since most of the work is done in Rust.


https://user-images.githubusercontent.com/2107635/190881464-31855641-a5b7-4fe0-aa7a-86e77995628a.mp4


## Running locally

cargo tauri dev

