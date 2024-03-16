-- This file should undo anything in `up.sql`
DROP TABLE invoice_attachments;

DROP TABLE invoice_rows;

DROP TABLE invoices;

DROP INDEX idx_invoices_status;

DROP INDEX idx_invoices_creation_time;

DROP TABLE addresses;

DROP TYPE invoice_status;