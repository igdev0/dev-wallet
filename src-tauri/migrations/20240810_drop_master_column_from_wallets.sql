-- Add migration script here
-- Step 1: Create a new table without the "master" column
CREATE TABLE wallets_new AS 
SELECT id, name, seed, created_at, password -- include all columns except 'master'
FROM wallets;

-- Step 2: Drop the old "wallets" table
DROP TABLE wallets;

-- Step 3: Rename the new table to "wallets"
ALTER TABLE wallets_new RENAME TO wallets;