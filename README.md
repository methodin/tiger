# tiger
Data deployment management

# Commands

## Shite

```sh
tiger init TEST-442
tiger TEST-442 ls
tiger TEST-442 pre sql
tiger TEST-442 post sql
tiger TEST-442 edit de19
tiger TEST-442 edit de19c58a7779036c3fba2c203d9ea88f
tiger TEST-442 clear
tiger -c ~/tiger.yaml up pre TEST-442
tiger -c ~/tiger.yaml down pre TEST-442
tiger -c ~/tiger.yaml up pre TEST-442 --run
tiger -c ~/tiger.yaml down pre TEST-442 --run
tiger -c ~/tiger.yaml up pre TEST-442 TEST-443 TEST-444
```
