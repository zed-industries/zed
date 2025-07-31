alter table billing_customers
add column has_overdue_invoices bool not null default false;
