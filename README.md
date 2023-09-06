# Postgres Database with Rust Backend using SQLx

## I am trying to learn Rust

### Database Setup

Use the Makefile commands to setup a Postgres database using Docker:

1. `make postgres` will run a new Docker container called *postgres-rs*

    If you are using a database management tool like [TablePlus](https://tableplus.com), then you can connect to the newly created database container.

    To create the connection using this example, the name of the PostgreSQL database is *postgres-rs*, the host/socket is localhost (127.0.0.1), the port is *2345* (this can be changed to 5432 which is the default port for PostgreSQL, but I already have a container at 5432 on my machine, so I changed it), the user is *root*, the password is *secret*, and the database is *postgres-rs*.

    ![TablePlusConnectionDemo](./tableplus-connection.png)

    You can also access the PostgreSQL console from within the Docker container.

    To end off, once you are connected or able to view the database and its tables, you should see there are no tables yet. We will migrate the database to add the initial tables.

2. `make mup` will run a SQLx database migration to create the tables.

    Now refresh or check the database and you should see the newly created product table with the columns provided in **[migrations/0001_product_table.up.sql](migrations/0001_product_table.up.sql)**.
