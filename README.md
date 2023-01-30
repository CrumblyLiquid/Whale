# Whale

Web application for practising vocabulary written with Rust and Yew.
It consists of `whale` lib which provides shared structs and functionality, `server` built with axum to host our API and `app` which houses the Yew application.
Finally I've provided some scripts to make converting between different file formats easier.

## Running Whale

1) Clone repository
2) Start server (navigate into the server folder and run `cargo run`)
3) Start serving the app (navigate into the app folder and run `trunk serve`)

## Scripts

I've provided some helpful scripts to check and convert between different files (text, json, docx, xlsx, ...)
