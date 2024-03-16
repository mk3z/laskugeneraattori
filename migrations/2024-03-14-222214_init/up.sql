-- Enum for invoice status
CREATE TYPE invoice_status AS ENUM ('open', 'accepted', 'paid', 'cancelled');

-- Addresses table
CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    street VARCHAR(128) NOT NULL,
    city VARCHAR(128) NOT NULL,
    zip VARCHAR(128) NOT NULL,
    CONSTRAINT no_duplicates UNIQUE (street, city, zip)
);

CREATE TABLE invoices (
    id SERIAL PRIMARY KEY,
    status invoice_status NOT NULL DEFAULT 'open',
    creation_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    recipient_name VARCHAR(128) NOT NULL,
    recipient_email VARCHAR(128) NOT NULL,
    bank_account_number VARCHAR(128) NOT NULL,
    address_id INT NOT NULL,
    FOREIGN KEY (address_id) REFERENCES addresses(id)
);

CREATE INDEX idx_invoices_status ON invoices(status);

CREATE INDEX idx_invoices_creation_time ON invoices(creation_time);

-- Invoice rows table
CREATE TABLE invoice_rows (
    id SERIAL PRIMARY KEY,
    invoice_id INT NOT NULL,
    product VARCHAR(128) NOT NULL,
    quantity INT NOT NULL,
    unit VARCHAR(128) NOT NULL,
    unit_price INT NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
);

-- Invoice attachments table
CREATE TABLE invoice_attachments (
    id SERIAL PRIMARY KEY,
    invoice_id INT NOT NULL,
    filename VARCHAR(128) NOT NULL,
    hash VARCHAR(64) NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
);