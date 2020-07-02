# OpenPlasma
Golang implementation of Ethereum zk-plasma (second level payment protocol).

## Run

### Database
Create user and database:
```
CREATE DATABASE openplasma;
CREATE USER openplasma WITH PASSWORD 'openplasma' SUPERUSER;
```
Create admin page tables:
```
psql -d openplasma -f .../OpenPlasma/plasma/models/admin/admin_db.pgsql
```
### Plasma
In the root directory run following command:
```
go run cmd/plasma/main.go
```