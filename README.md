# Modolumia

A work-in-progress website for sharing Mixolumia mod files

Uses Actix-Web, couchdb, and Reactjs

Very work-in-progress

## Setup
create a `.env` file that sets up the environment variables `DATABASE_URL` and `DATABASE_AUTHORIZATION` with the correct information<br>
startup couchdb<br>
build react frontend<br>
run `cargo run`<br>



## To Do
### Backend
 - [x] Interface for communicating between webserver and couchdb
 - [ ] API For creating, getting, and editing posts
 - [ ] Add accounts via google, discord, and possibly more oauth
 - [ ] Serve up dynamic frontend html
 - [ ] Track statistics (views, downloads, ect...)
### Frontend
 - [ ] Page template
 - [ ] Communicate with webserver to establish logins
 - [ ] Search page
 - [ ] Much more