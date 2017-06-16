# tiger
Data deployment management

# Commands

```sh
tiger init TEST-442
tiger TEST-442 ls
tiger TEST-442 add pre sql
tiger TEST-442 add post sql
tiger TEST-442 edit de19
tiger TEST-442 edit de19c58a7779036c3fba2c203d9ea88f
tiger -c ~/tiger.yaml up pre TEST-442
tiger -c ~/tiger.yaml down pre TEST-442
tiger -c ~/tiger.yaml up pre TEST-442 --run
tiger -c ~/tiger.yaml down pre TEST-442 --run
tiger -c ~/tiger.yaml up pre TEST-442 TEST-443 TEST-444
```
