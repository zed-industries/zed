# vim: set ft=dockerfile:
ARG VERSION
FROM mcr.microsoft.com/mssql/server:${VERSION}

# Create a config directory
RUN mkdir -p /usr/config
WORKDIR /usr/config

# Bundle config source
COPY mssql/entrypoint.sh /usr/config/entrypoint.sh
COPY mssql/configure-db.sh /usr/config/configure-db.sh
COPY mssql/setup.sql /usr/config/setup.sql

# Grant permissions for to our scripts to be executable
USER root
RUN chmod +x /usr/config/entrypoint.sh
RUN chmod +x /usr/config/configure-db.sh

ENTRYPOINT ["/usr/config/entrypoint.sh"]
