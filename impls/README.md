# Services
## Folder Layout
Each implementation will live in a folder in this directory. The format for the folder
names will be `language_framework`. So for instance, if the service was built using
[rust](https://www.rust-lang.org/) with the [actix](https://actix.rs/) framework, then
the folder structure would look like below:

```
impls
│   README.md
│
└─── rust_actix
│       README.md
│       etc...
│
└─── kotlin_springboot
│       README.md
│       etc...
│
└─── etc
│       README.md
│
...
```

The contents of each folder should contain a README which outlines the build steps
for the service alongside all the code. The output from the build should be a
docker image which will be uploaded to a docker registry _to be defined_. Will
probably try and incorporate github actions to perform the builds... grand plans...

## Database
The database will be made available to the apps to connect to when the comparison
tests are being run. The connection details will be made available to the service
via env variables which are probably self explanatory
```
DB_HOST
DB_NAME
DB_USER
DB_PSWD
```