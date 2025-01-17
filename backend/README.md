# Setup the database

After installing PostgreSQL
Run the following in `psql`:

```sql
CREATE ROLE pdsdi WITH LOGIN PASSWORD 'pdsdi';
CREATE DATABASE clothe_match OWNER pdsdi;
```

## Tls certificate

For testing purposes, we recommend using [mkcert](https://github.com/FiloSottile/mkcert):

```bash
mkdir certs
cd certs
mkcert -install
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```
