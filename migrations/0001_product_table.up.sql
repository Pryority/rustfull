CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE products (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    sku INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    price INTEGER NOT NULL,
    sale_price INTEGER NOT NULL
);

CREATE UNIQUE INDEX product_sku ON products (sku);
