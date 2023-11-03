# task-notes-backend
Task lists categorized by project

Backend api for task notes.
Uses diesel-rs and actix-web.

### Installation

Install and configure postgres:

``` sudo apt-get install libpq-dev ```

``` sudo apt-get install postgresql postgresql-contrib  ```

``` createuser --interactive --pwprompt ```

``` echo DATABASE_URL=postgres://username:password@localhost/taskmanager > .env ```

Install diesel:

``` cargo install diesel_cli --no-default-features --features postgres```

Run migrations: 

```diesel migration run```

