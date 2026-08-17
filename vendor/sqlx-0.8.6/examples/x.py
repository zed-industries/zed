#!/usr/bin/env python3

import sys
import os
from os import path

# base dir of sqlx workspace
dir_workspace = path.dirname(path.dirname(path.realpath(__file__)))

# dir of tests
dir_tests = path.join(dir_workspace, "tests")

# extend import path to tests/
sys.path.append(dir_tests)

import subprocess
import time
import argparse
from docker import start_database

parser = argparse.ArgumentParser()
parser.add_argument("-p", "--project")
parser.add_argument("-l", "--list-projects", action="store_true")

argv, unknown = parser.parse_known_args()


def run(command, env=None, cwd=None, display=None):
    if display:
        print(f"\x1b[93m $ {display}\x1b[0m")

    else:
        print(f"\x1b[93m $ {command}\x1b[0m")

    res = subprocess.run(
        command.split(" "),
        env=dict(**os.environ, **env),
        cwd=cwd,
    )

    if res.returncode != 0:
        sys.exit(res.returncode)


def sqlx(command, url, cwd=None):
    run(f"cargo --quiet run -p sqlx-cli --bin sqlx -- {command}", cwd=cwd, env={"DATABASE_URL": url},
        display=f"sqlx {command}")


def project(name, database=None, driver=None):
    if argv.list_projects:
        print(f"{name}")
        return

    if argv.project and name != argv.project:
        return

    print(f"\x1b[2m # {name}\x1b[0m")

    env = {}

    cwd = path.join(dir_workspace, "examples", name)

    if database is not None:
        database_url = start_database(driver, database, cwd=cwd)
        env["DATABASE_URL"] = database_url

        # show the database url
        print(f"\x1b[94m @ {database_url}\x1b[0m")

        # database drop (if exists)
        sqlx("db drop -y", database_url, cwd=cwd)

        # database create
        sqlx("db create", database_url, cwd=cwd)

        # migrate
        sqlx("migrate run", database_url, cwd=cwd)

    # check
    run("cargo check", cwd=cwd, env=env)


# todos
project("mysql/todos", driver="mysql_8", database="todos")
project("postgres/todos", driver="postgres_12", database="todos")
project("sqlite/todos", driver="sqlite", database="todos.db")
