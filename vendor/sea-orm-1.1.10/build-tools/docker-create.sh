# Some Common Docker Commands You Might Need (use with caution)
# 
# Delete all containers
# $ docker rm -f $(docker ps -a -q)
# 
# Delete all volumes
# $ docker volume rm $(docker volume ls -q)
# 
# Delete all images
# $ docker image rm $(docker image ls -q)

# Setup MariaDB

docker run \
    --name "mariadb-10.6" \
    --env MYSQL_DB="mysql" \
    --env MYSQL_USER="sea" \
    --env MYSQL_PASSWORD="sea" \
    --env MYSQL_ALLOW_EMPTY_PASSWORD="yes" \
    --env MYSQL_ROOT_PASSWORD="root" \
    -d -p 3306:3306 mariadb:10.6
docker stop "mariadb-10.6"

docker run \
    --name "mariadb-10.5" \
    --env MYSQL_DB="mysql" \
    --env MYSQL_USER="sea" \
    --env MYSQL_PASSWORD="sea" \
    --env MYSQL_ALLOW_EMPTY_PASSWORD="yes" \
    --env MYSQL_ROOT_PASSWORD="root" \
    -d -p 3306:3306 mariadb:10.5
docker stop "mariadb-10.5"

docker run \
    --name "mariadb-10.4" \
    --env MYSQL_DB="mysql" \
    --env MYSQL_USER="sea" \
    --env MYSQL_PASSWORD="sea" \
    --env MYSQL_ALLOW_EMPTY_PASSWORD="yes" \
    --env MYSQL_ROOT_PASSWORD="root" \
    -d -p 3306:3306 mariadb:10.4
docker stop "mariadb-10.4"

# Setup MySQL

docker run \
    --name "mysql-8.0" \
    --env MYSQL_DB="mysql" \
    --env MYSQL_USER="sea" \
    --env MYSQL_PASSWORD="sea" \
    --env MYSQL_ALLOW_EMPTY_PASSWORD="yes" \
    --env MYSQL_ROOT_PASSWORD="root" \
    -d -p 3306:3306 mysql:8.0
docker stop "mysql-8.0"

docker run \
    --name "mysql-5.7" \
    --env MYSQL_DB="mysql" \
    --env MYSQL_USER="sea" \
    --env MYSQL_PASSWORD="sea" \
    --env MYSQL_ALLOW_EMPTY_PASSWORD="yes" \
    --env MYSQL_ROOT_PASSWORD="root" \
    -d -p 3306:3306 mysql:5.7
docker stop "mysql-5.7"

# Setup PostgreSQL

docker run \
    --name "postgres-vector-14" \
    --env POSTGRES_USER="root" \
    --env POSTGRES_PASSWORD="root" \
    -d -p 5432:5432 pgvector/pgvector:pg14
docker stop "postgres-vector-14"

docker run \
    --name "postgres-14" \
    --env POSTGRES_USER="root" \
    --env POSTGRES_PASSWORD="root" \
    -d -p 5432:5432 postgres:14
docker stop "postgres-14"

docker run \
    --name "postgres-13" \
    --env POSTGRES_USER="root" \
    --env POSTGRES_PASSWORD="root" \
    -d -p 5432:5432 postgres:13
docker stop "postgres-13"

docker run \
    --name "postgres-12" \
    --env POSTGRES_USER="root" \
    --env POSTGRES_PASSWORD="root" \
    -d -p 5432:5432 postgres:12
docker stop "postgres-12"

docker run \
    --name "postgres-11" \
    --env POSTGRES_USER="root" \
    --env POSTGRES_PASSWORD="root" \
    -d -p 5432:5432 postgres:11
docker stop "postgres-11"
