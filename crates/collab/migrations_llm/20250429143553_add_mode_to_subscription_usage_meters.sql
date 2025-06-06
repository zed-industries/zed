alter table subscription_usage_meters
    add column mode text not null default 'normal';

drop index uix_subscription_usage_meters_on_subscription_usage_model;

create unique index uix_subscription_usage_meters_on_subscription_usage_model_mode on subscription_usage_meters (subscription_usage_id, model_id, mode);
