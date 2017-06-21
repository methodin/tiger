# tiger
Data deployment management

# Commands during development

## Initialize a project
To create a new project in the current directory for a Jira ticket TEST-442 you would run the following:
```sh
tiger init TEST-442
```

## List changes
Once a project has been created there are a series of commands under that project that you can run. A very useful one is ls that lists all changes:
```sh
tiger TEST-442 ls
```

## Add a change
You can add a change to a project by executing a pre or post command that will generate a new up/down for either a pre-deploy task or a post-deploy task
```sh
tiger TEST-442 pre sql
tiger TEST-442 post sql
```

## Edit scripts
There is a shortcut command to edit both up/down files for a changeset. The hash provided can be the entire hash or any amount of characters at the beginning 
```sh
tiger TEST-442 edit de19
tiger TEST-442 edit de19c58a7779036c3fba2c203d9ea88f
```

## Remove changes
You can remove a single change or all changes in the project:
```sh
tiger TEST-442 rm de19
tiger TEST-442 clear
```

## Simulating scripts
After you have created your changes you can simulate what an up/down event would look like:
```sh
tiger MAG-655 simulate up
tiger MAG-655 simulate down
```

# Running changes 

## Packaging a project
When all your work is done for a project and you are ready to ship, you must package it up into a binary that will upload to s3. These binaries are used by the subsequent run commands. Once you package a project it cannot be uploaded using the same name - it's recommended to use a version # for subsequent versions:
*Note % in the name is replaced by the project name*
```sh
tiger -c ~/tiger.yaml TEST-442 package %
tiger -c ~/tiger.yaml TEST-442 package %-1
tiger -c ~/tiger.yaml TEST-442 package %-2
```

## Non-commit run-through
You can check all changes that are to be staged by simulating an up or down in a pre or post world and provide one or more projects to load:
```sh
tiger -c ~/tiger.yaml up pre TEST-442
tiger -c ~/tiger.yaml up post TEST-442 TEST-443
tiger -c ~/tiger.yaml down post TEST-442 TEST-443
tiger -c ~/tiger.yaml down pre TEST-442
```

## Commit changes
Once the run through looks good you can commit all changes by attaching the --run flag:
```sh
tiger -c ~/tiger.yaml up pre TEST-442 TEST-443 --run
tiger -c ~/tiger.yaml up post TEST-442 TEST-443 --run
tiger -c ~/tiger.yaml down post TEST-442 TEST-443 --run
tiger -c ~/tiger.yaml down pre TEST-442 TEST-443 --run
```
