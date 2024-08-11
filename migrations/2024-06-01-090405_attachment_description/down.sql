-- This file should undo anything in `up.sql`
ALTER TABLE invoice_attachments
DROP COLUMN description;
