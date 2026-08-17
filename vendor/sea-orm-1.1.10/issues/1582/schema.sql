CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table without any dependencies
CREATE TABLE IF NOT EXISTS public.table_1 (
  id uuid DEFAULT public.uuid_generate_v4() NOT NULL PRIMARY KEY
);

-- Table that depends on table_1
CREATE TABLE IF NOT EXISTS public.table_2 (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    table_1_id uuid NOT NULL REFERENCES public.table_1(id),
    -- Add constraints / uniques here if needed. Will auto apply to all partitions.
    PRIMARY KEY (id, table_1_id)
)
PARTITION BY HASH (table_1_id);

-- Generate partition tables.
-- No clue if this works on other SQL implementations than PostgreSQL.
DO $$
  DECLARE
    counter integer := 0;
  BEGIN
    WHILE counter < 128
    LOOP
      EXECUTE('CREATE TABLE IF NOT EXISTS public.table_2_p_hash_p' || counter || ' PARTITION OF public.table_2 FOR VALUES WITH (MODULUS 128, REMAINDER ' || counter || ');');
      counter := counter + 1;
    END LOOP;
END$$;

-- Table that depends on table_1 and table_2.
-- Foreign keys to table_2 are through the partitions
CREATE TABLE IF NOT EXISTS public.table_3 (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL PRIMARY KEY,
    -- FK to table_1 only needed so that we can FK to table_2
    table_1_id uuid REFERENCES public.table_1(id) NOT NULL,
    table_2_id uuid NOT NULL,
    FOREIGN KEY (table_2_id, table_1_id) REFERENCES public.table_2(id, table_1_id)
);

