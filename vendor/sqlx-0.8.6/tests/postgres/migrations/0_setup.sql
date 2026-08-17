-- `gen_random_uuid()` wasn't added until Postgres 13
create extension if not exists "uuid-ossp";
