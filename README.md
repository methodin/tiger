# tiger
Data deployment management

# Dependencies
Tiger has a hard dependency on AWS S3 for storage of packaged binaries and MySQL for actual SQL migrations. That may change in the future and is certainly a blocker for anyone not using either of these two tools. If you have another toolset you would like to see supported ass an Issue and I will review.

# Using tiger
It's recommended to use tiger with Docker to avoid having to worry about specific dependencies. You must first build the docker container from within the tiger git checkout directory:

```sh
docker build -t tiger .
```

# For developers
For developers using tiger in their projects it's recommended to create the following aliases in your .bash_aliases file:

```
tiger() {
    local yaml="/home/ec2-user/tiger.yaml";
    local aws="/home/.aws";
    local mysql="127.0.0.1";
    docker run --rm -it -v ${PWD}:/tiger -v $yaml:/tiger/tiger.yaml -v $aws:/home/rust/.aws --net host --add-host mysql:$mysql  tiger -c /tiger/tiger.yaml $@;
}
tiger-edit() {
    vim -p `tiger $1 files $2`
}
```
The first alias will map any current directory to the working directory in the tiger docker instance and avoid having to install rust/cargo etc... The second alias is a convenience for editing the scripts for a particular project and changeset. It's use is:

```
tiger-edit TEST-442 f9a
```

Which will open both up/down files in a vim editor.

# For production
In production you would want a utility like Jenkins to simply execute the docker run for whatever commands you are executing.

An example of the straight docker run is as follows:

```sh
docker run --rm -it \
  -v ~/tiger.yaml:/tiger/tiger.yaml \
  -v ~/.aws:/home/rust/.aws \
  --net host \
  --add-host mysql:127.0.0.1 \
  tiger -c /tiger/tiger.yaml up pre TEST-442
```

You can see it generally reflects the dev alias above except you'd be hardcoding the value depending on your build tool.

## Initialize a project
To create a new project in the current directory for a Jira ticket TEST-442 you would run the following:
```sh
tiger init TEST-442

> Creating project TEST-442 in current dir: /home/ec2-user/Work/projects/test
> Successfully created project file /home/ec2-user/Work/projects/test/tiger/TEST-442/project.json
```

## List changes
Once a project has been created there are a series of commands under that project that you can run. A very useful one is ls that lists all changes:
```sh
tiger TEST-442 ls

> Current changes in project:
> 
> |------------|------------|----------------------------------|
> | Timing     | Type       | Hash                             |
> |------------|------------|----------------------------------|
> | pre        | sql        | f9a107647301283c0d4123d886d9c45f |
> | post       | sql        | 22febbdb5ee79725257bdc173292e832 |
> |------------|------------|----------------------------------|
```

## Add a change
You can add a change to a project by executing a pre or post command that will generate a new up/down for either a pre-deploy task or a post-deploy task
```sh
tiger TEST-442 pre sql

> Creating new change f9a107647301283c0d4123d886d9c45f
> Creating new up file /home/ec2-user/Work/projects/test/tiger/TEST-442/f9a107647301283c0d4123d886d9c45f/up.sql
> Creating new up file /home/ec2-user/Work/projects/test/tiger/TEST-442/f9a107647301283c0d4123d886d9c45f/down.sql
> Successfully created project file /home/ec2-user/Work/projects/test/tiger/TEST-442/project.json

tiger TEST-442 post sql

> Creating new change 22febbdb5ee79725257bdc173292e832
> Creating new up file /home/ec2-user/Work/projects/test/tiger/TEST-442/22febbdb5ee79725257bdc173292e832/up.sql
> Creating new up file /home/ec2-user/Work/projects/test/tiger/TEST-442/22febbdb5ee79725257bdc173292e832/down.sql
> Successfully created project file /home/ec2-user/Work/projects/test/tiger/TEST-442/project.json
```

## List scripts
You can use the files command to list the up/down files for a changset. This output is on one line for ease of pipping to an editor of your choice
tiger TEST-442 files f9a1
tiger TEST-442 files f9a107647301283c0d4123d886d9c45f
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
tiger TEST-442 simulate up

> Pre-deploy changes: 1
> Post-deploy changes: 1
> 
> PRE SCRIPTS
> ----------------------------------------------------------------------------------------------------
> ALTER TABLE `test` ADD COLUMN `new_column` VARCHAR(100);
> 
> ----------------------------------------------------------------------------------------------------
> 
> POST SCRIPTS
> ----------------------------------------------------------------------------------------------------
> ALTER TABLE `test` ADD COLUMN `new_column2` VARCHAR(100);
> 
> ----------------------------------------------------------------------------------------------------
> Deployment complete

tiger TEST-442 simulate down

> Pre-deploy changes: 1
> Post-deploy changes: 1
> 
> PRE SCRIPTS
> ----------------------------------------------------------------------------------------------------
> ALTER TABLE `test` DROP COLUMN;
> 
> ----------------------------------------------------------------------------------------------------
> 
> POST SCRIPTS
> ----------------------------------------------------------------------------------------------------
> ALTER TABLE `test` DROP COLUMN `new_column2`;
> 
> ----------------------------------------------------------------------------------------------------
> Deployment complete
```

# Running changes 

## Packaging a project
When all your work is done for a project and you are ready to ship, you must package it up into a binary that will upload to s3. These binaries are used by the subsequent run commands. Once you package a project it cannot be uploaded using the same name - it's recommended to use a version # for subsequent versions:

*Note: % in the name is replaced by the project name*

```sh
tiger -c ~/tiger.yaml TEST-442 package %

> Packaging project file TEST-442.bin
> Packaging complete... uploading to s3
> Successfully uploaded package to s3

tiger -c ~/tiger.yaml TEST-442 package %-1

> Packaging project file TEST-442-1.bin
> Packaging complete... uploading to s3
> Successfully uploaded package to s3

tiger -c ~/tiger.yaml TEST-442 package %-2

> Packaging project file TEST-442-2.bin
> Packaging complete... uploading to s3
> Successfully uploaded package to s3
```

## Non-commit run-through
You can check all changes that are to be staged by simulating an up or down in a pre or post world and provide one or more projects to load:
```sh
tiger -c ~/tiger.yaml up pre TEST-442

> Running in simulation mode
> Connecting to sql server
> Downloading packages
> Executing the following SQL code:
> ALTER TABLE `test` ADD COLUMN `new_column` VARCHAR(100);

> Migration complete

tiger -c ~/tiger.yaml up post TEST-442 TEST-443
tiger -c ~/tiger.yaml down post TEST-442 TEST-443
tiger -c ~/tiger.yaml down pre TEST-442
```

## Commit changes
Once the run through looks good you can commit all changes by attaching the --run flag:
```sh
tiger -c ~/tiger.yaml up pre TEST-442 TEST-443 --run

> Connecting to sql server
> Downloading packages
> Executing the following SQL code:
> ALTER TABLE `magneto`.`tiger_test` ADD COLUMN `new_column` VARCHAR(100);
> 
> Success
> Migration complete

tiger -c ~/tiger.yaml up post TEST-442 TEST-443 --run
tiger -c ~/tiger.yaml down post TEST-442 TEST-443 --run
tiger -c ~/tiger.yaml down pre TEST-442 TEST-443 --run
```
