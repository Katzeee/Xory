```bash
cargo install sea-orm-cli
sea-orm-cli migrate fresh
sea-orm-cli generate entity -o common/src/entity --with-serde both
```


```sql
CREATE USER 'test'@'%' IDENTIFIED BY 'password';
CREATE DATABASE xory;
GRANT ALL PRIVILEGES ON xory.* TO 'test'@'%';
FLUSH PRIVILEGES;
```