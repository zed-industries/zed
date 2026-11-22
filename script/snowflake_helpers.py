"""
Run read-only queries through Snowflake's SQL API, authenticating as a
service user with a key pair.

Exists so CI scripts don't need snowflake-connector-python and its heavy
dependency tree for a few small queries.

Requires:
    requests, cryptography
"""

import base64
import hashlib
import json
import os
import time

import requests
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

TOKEN_LIFETIME_SECONDS = 300
# Below the 45s Snowflake waits before answering 202 (still running), so a
# statement always ends in a final answer and we never have to poll.
STATEMENT_TIMEOUT_SECONDS = 40
REQUEST_TIMEOUT_SECONDS = 60
MAX_RETRIES = 3


def _base64url(data):
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def _key_pair_token(account, user, private_key):
    public_key = private_key.public_key().public_bytes(
        serialization.Encoding.DER, serialization.PublicFormat.SubjectPublicKeyInfo
    )
    fingerprint = base64.b64encode(hashlib.sha256(public_key).digest()).decode()
    qualified_user = f"{account}.{user}"
    issued_at = int(time.time())
    header = {"alg": "RS256", "typ": "JWT"}
    claims = {
        "iss": f"{qualified_user}.SHA256:{fingerprint}",
        "sub": qualified_user,
        "iat": issued_at,
        "exp": issued_at + TOKEN_LIFETIME_SECONDS,
    }
    signing_input = ".".join(
        _base64url(json.dumps(part).encode()) for part in (header, claims)
    )
    signature = private_key.sign(
        signing_input.encode(), padding.PKCS1v15(), hashes.SHA256()
    )
    return f"{signing_input}.{_base64url(signature)}"


def query_snowflake(statement):
    """Run a statement and return its rows, each a list of strings."""
    account = os.environ["SNOWFLAKE_ACCOUNT"].upper()
    user = os.environ["SNOWFLAKE_USER"].upper()
    private_key = serialization.load_pem_private_key(
        os.environ["SNOWFLAKE_PRIVATE_KEY"].encode(), password=None
    )
    session = requests.Session()
    # Resending is safe because the module only runs read-only statements.
    retry = Retry(
        total=MAX_RETRIES,
        backoff_factor=2,
        status_forcelist=[429, 500, 502, 503, 504],
        allowed_methods=None,
        raise_on_status=False,
    )
    session.mount("https://", HTTPAdapter(max_retries=retry))

    response = session.post(
        f"https://{account}.snowflakecomputing.com/api/v2/statements",
        headers={
            "Authorization": f"Bearer {_key_pair_token(account, user, private_key)}",
            "Accept": "application/json",
            "User-Agent": "zed-industries/zed scripts",
        },
        json={
            "statement": statement,
            "timeout": STATEMENT_TIMEOUT_SECONDS,
            "role": os.environ["SNOWFLAKE_ROLE"],
            "warehouse": os.environ["SNOWFLAKE_WAREHOUSE"],
        },
        timeout=REQUEST_TIMEOUT_SECONDS,
    )
    if response.status_code != 200:
        raise RuntimeError(
            f"Snowflake SQL API returned {response.status_code}: {response.text}"
        )
    result = response.json()
    rows = result["data"]
    if len(rows) != result["resultSetMetaData"]["numRows"]:
        raise RuntimeError("Snowflake result spans several partitions, unsupported")
    return rows
