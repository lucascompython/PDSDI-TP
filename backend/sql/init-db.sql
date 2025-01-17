DO $$ BEGIN IF NOT EXISTS (
    SELECT
    FROM pg_catalog.pg_roles
    WHERE rolname = 'pdsdi'
) THEN CREATE ROLE pdsdi WITH LOGIN PASSWORD 'pdsdi';
END IF;
END $$;
DO $$ BEGIN IF NOT EXISTS (
    SELECT
    FROM pg_database
    WHERE datname = 'clothe_match'
) THEN CREATE DATABASE clothe_match OWNER pdsdi;
END IF;
END $$;