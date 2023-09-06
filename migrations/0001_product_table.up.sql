CREATE TABLE product (
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    sku VARCHAR NOT NULL,
    quantity INTEGER NOT NULL,
    price INTEGER NOT NULL,
    sale_price INTEGER NOT NULL
);

CREATE UNIQUE INDEX product_sku ON product (sku);
