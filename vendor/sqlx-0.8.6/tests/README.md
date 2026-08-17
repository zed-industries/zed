

### Running Tests
SQLx uses docker to run many compatible database systems for integration testing. You'll need to [install docker](https://docs.docker.com/engine/) to run the full suite. You can validate your docker installation with:
    
    $ docker run hello-world

Start the databases with `docker-compose` before running tests: 

    $ docker-compose up

Run all tests against all supported databases using:

    $ ./x.py

If you see test failures, or want to run a more specific set of tests against a specific database, you can specify both the features to be tests and the DATABASE_URL. e.g.

    $ DATABASE_URL=mysql://root:password@127.0.0.1:49183/sqlx cargo test --no-default-features --features macros,offline,any,all-types,mysql,runtime-async-std-native-tls
