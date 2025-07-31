alter table users alter column github_user_id set not null;

drop index index_users_on_github_user_id;
create unique index uix_users_on_github_user_id on users (github_user_id);
