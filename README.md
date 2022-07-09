# Example web app with DDD principles in mind
This is a work in progress DDD example web app and may or may not, become my future blueprint to set up a new web app.
It's highly work in progress and by far not complete. Also, I'm relatively new to Rust, so you may see some strange things here. ;-)


### Requirements
---
- Docker / Docker compose

### Local development 
--- 
- Copy .env.dist to .env and adjust values if necessary
- Start database service with `docker-compose up -d`
- Start app with `cargo run`