# Setup the database

After installing PostgreSQL
Run the following in `psql`:

```sql
CREATE ROLE pdsdi WITH LOGIN PASSWORD 'pdsdi';
CREATE DATABASE clothe_match OWNER pdsdi;
```
