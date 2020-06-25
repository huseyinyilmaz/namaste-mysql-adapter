Namaste mysql adapter
=====================

```
huseyin@admins-MBP namaste-shim % echo "SELECT * FROM foo" | mysql -h 127.0.0.1 --table
+------+-------------------+
| id   | name              |
+------+-------------------+
|    1 | Huseyin           |
|    2 | Ayse              |
|    3 | Sila              |
|    4 | Mert              |
|    5 | SELECT * FROM foo |
+------+-------------------+
huseyin@admins-MBP namaste-shim %
```