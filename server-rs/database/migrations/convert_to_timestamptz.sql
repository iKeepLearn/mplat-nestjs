DO $$
DECLARE
    rec RECORD;
BEGIN
    FOR rec IN 
        SELECT table_name, column_name 
        FROM information_schema.columns 
        WHERE table_schema = 'public' 
          AND data_type = 'timestamp without time zone'
    LOOP
        EXECUTE format('ALTER TABLE %I ALTER COLUMN %I TYPE TIMESTAMPTZ', 
                      rec.table_name, rec.column_name);
        RAISE NOTICE 'Converted %.% to TIMESTAMPTZ', rec.table_name, rec.column_name;
    END LOOP;
END $$;