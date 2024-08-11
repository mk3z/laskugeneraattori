-- Your SQL goes here
ALTER TABLE invoice_attachments
ADD COLUMN description VARCHAR(512) NOT NULL DEFAULT '';
