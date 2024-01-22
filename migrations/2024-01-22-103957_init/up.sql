-- Your SQL goes here
CREATE TYPE invoice_status AS ENUM ('open', 'accepted', 'paid');

CREATE TABLE parties(
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    street VARCHAR(128) NOT NULL,
    city VARCHAR(128) NOT NULL,
    zip VARCHAR(128) NOT NULL,
    bank_account VARCHAR(128) NOT NULL,
    -- NOTE: This constraint is a bit heavy
    CONSTRAINT no_duplicates
        UNIQUE (name, street, city, zip, bank_account)
);

CREATE TABLE invoices(
    id SERIAL PRIMARY KEY,
    status invoice_status NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    counter_party_id int NOT NULL,
    due_date DATE NOT NULL,
    CONSTRAINT fk_party
        FOREIGN KEY(counter_party_id)
            REFERENCES parties(id)
);

CREATE TABLE invoice_rows(
    id SERIAL PRIMARY KEY,
    invoice_id int NOT NULL,
    product VARCHAR(128) NOT NULL,
    quantity int NOT NULL,
    unit VARCHAR(128) NOT NULL,
    unit_price int NOT NULL,
    CONSTRAINT fk_invoice
        FOREIGN KEY(invoice_id)
            REFERENCES invoices(id)
            ON DELETE CASCADE
);

CREATE TABLE invoice_attachments(
    id SERIAL PRIMARY KEY,
    invoice_id int NOT NULL,
    filename VARCHAR(128) NOT NULL,
    hash VARCHAR(64) NOT NULL,
    CONSTRAINT fk_invoice
        FOREIGN KEY(invoice_id)
            REFERENCES invoices(id)
            ON DELETE CASCADE
);
