package xtls

import (
	"crypto/tls"
	"crypto/x509"
	"errors"
	"fmt"
	"os"
)

type Config struct {
	Enabled bool `json:"enabled" yaml:"enabled"`

	// Skip server certificate and domain verification.
	Insecure bool `json:"insecure" yaml:"insecure" config:"allowempty"`

	// Server name indication for TLS.
	ServerName string `json:"serverName" yaml:"server_name" config:"allowempty"`

	// File containing trusted root certificates for verifying the server.
	CACertFile string `json:"caCertFile" yaml:"ca_cert_file" config:"allowempty"`

	// File containing client certificate (public key), to present to the
	// server. Must also provide @ClientKey option.
	ClientCertFile string `json:"clientCertFile" yaml:"client_cert_file" config:"allowempty"`

	// File containing client private key, to present to the server.
	// Must also provide @ClientCert option.
	ClientKeyFile string `json:"clientKeyFile" yaml:"client_key_file" config:"allowempty"`
}

var ErrFailedToLoadCACert = errors.New("failed to load CACertificate")

func (c *Config) ClientTLSConfig() (*tls.Config, error) {
	tlsConf := tls.Config{
		MinVersion: tls.VersionTLS12,
	}

	if c.ClientCertFile != "" {
		// Load the client certificates from disk
		certificate, err := tls.LoadX509KeyPair(c.ClientCertFile, c.ClientKeyFile)
		if err != nil {
			return nil, fmt.Errorf("could not load client key pair: %w", err)
		}

		tlsConf.Certificates = []tls.Certificate{certificate}
	}

	if c.Insecure {
		// #nosec G402
		tlsConf.InsecureSkipVerify = true
	}

	if c.ServerName != "" {
		tlsConf.ServerName = c.ServerName
	}

	if c.CACertFile != "" {
		// Create a certificate pool from the certificate authority
		certPool := x509.NewCertPool()

		ca, err := os.ReadFile(c.CACertFile)
		if err != nil {
			return nil, fmt.Errorf("could not read ca certificate: %w", err)
		}

		if ok := certPool.AppendCertsFromPEM(ca); !ok {
			return nil, ErrFailedToLoadCACert
		}

		tlsConf.RootCAs = certPool
	}

	return &tlsConf, nil
}
