Different databases installable via Docker & Docker-compose

Displays a menu in the terminal for creating, deleting, starting DBs via docker-compose:

```cargo run -- --dir=$PWD```

What does it do?
- Select some Databases to start locally via terminal command: ```db```

What does it need?
- Ubuntu/Linux
- Docker and Docker-compose installed locally

How to use:
- Clone this repository
- Build binary ```cargo build --release``` and put the binary in your PATH-env
- Add a new ENV to your .bashrc pointing to your local repository with ```DB_DIR=...```
- Now you can create/start/stop/delete a DB via ```db``` in the terminal

Currently supported DBs:
- Mongodb
- Postgres
