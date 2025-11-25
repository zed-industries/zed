alter table billing_customers
    add column external_id text;

create unique index uix_billing_customers_on_external_id on billing_customers (external_id);
