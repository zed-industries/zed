import subprocess
import sys
import time
from os import path
import shutil

# base dir of sqlx workspace
dir_workspace = path.dirname(path.dirname(path.realpath(__file__)))

# dir of tests
dir_tests = path.join(dir_workspace, "tests")


# start database server and return a URL to use to connect
def start_database(driver, database, cwd):
    if driver == "sqlite":
        database = path.join(cwd, database)
        (base_path, ext) = path.splitext(database)
        new_database = f"{base_path}.test{ext}"
        shutil.copy(database, new_database)
        # short-circuit for sqlite
        return f"sqlite://{path.join(cwd, new_database)}"

    res = subprocess.run(
        ["docker-compose", "up", "-d", driver],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        cwd=dir_tests,
    )

    if res.returncode != 0:
        print(res.stderr, file=sys.stderr)

    if b"done" in res.stderr:
        time.sleep(30)

    # determine appropriate port for driver
    if driver.startswith("mysql") or driver.startswith("mariadb"):
        port = 3306

    elif driver.startswith("postgres"):
        port = 5432

    else:
        raise NotImplementedError

    # find port
    res = subprocess.run(
        ["docker", "inspect", f"-f='{{{{(index (index .NetworkSettings.Ports \"{port}/tcp\") 0).HostPort}}}}'",
         f"sqlx_{driver}_1"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        cwd=dir_tests,
    )

    if res.returncode != 0:
        print(res.stderr, file=sys.stderr)

    port = int(res.stdout[1:-2].decode())

    # need additional permissions to connect to MySQL when using SSL
    res = subprocess.run(
        ["docker", "exec", f"sqlx_{driver}_1", "mysql", "-u", "root", "-e", "GRANT ALL PRIVILEGES ON *.* TO 'root' WITH GRANT OPTION;"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        cwd=dir_tests,
    )

    if res.returncode != 0:
        print(res.stderr, file=sys.stderr)

    # do not set password in URL if authenticating using SSL key file
    if driver.endswith("client_ssl"):
        password = ""
    else:
        password = ":password"

    # construct appropriate database URL
    if driver.startswith("mysql") or driver.startswith("mariadb"):
        return f"mysql://root{password}@localhost:{port}/{database}"

    elif driver.startswith("postgres"):
        return f"postgres://postgres{password}@localhost:{port}/{database}"

    else:
        raise NotImplementedError
