# tiger
Data deployment management

# Commands

```sh
tiger init TEST-442
tiger TEST-442 data set pre ~/my-pre-deploy-script.sql
tiger TEST-442 data set post ~/my-post-deploy-script.sql
tiger TEST-442 ls
tiger TEST-442 simulate
tiger run ~/tiger.config.yaml TEST-442
```
