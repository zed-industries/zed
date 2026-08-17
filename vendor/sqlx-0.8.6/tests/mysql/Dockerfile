ARG IMAGE
FROM ${IMAGE}

# Copy SSL certificate (and key)
COPY certs/server.crt /etc/mysql/ssl/server.crt
COPY certs/ca.crt /etc/mysql/ssl/ca.crt
COPY keys/server.key /etc/mysql/ssl/server.key
COPY mysql/my.cnf /etc/mysql/my.cnf

# Fix permissions
RUN chown mysql:mysql /etc/mysql/ssl/server.crt /etc/mysql/ssl/server.key
RUN chmod 0600 /etc/mysql/ssl/server.crt /etc/mysql/ssl/server.key

# Create dir for secure-file-priv
RUN mkdir -p /var/lib/mysql-files
