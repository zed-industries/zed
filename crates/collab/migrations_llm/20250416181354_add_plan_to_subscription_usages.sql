alter table subscription_usages
    add column plan text not null;

create index ix_subscription_usages_on_plan on subscription_usages (plan);
