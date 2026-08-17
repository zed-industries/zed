-- https://rustsec.org/advisories/RUSTSEC-2024-0363.html
-- https://github.com/launchbadge/sqlx/issues/3440
CREATE TABLE injection_target(id BIGSERIAL PRIMARY KEY, message TEXT);
INSERT INTO injection_target(message) VALUES ('existing value');
