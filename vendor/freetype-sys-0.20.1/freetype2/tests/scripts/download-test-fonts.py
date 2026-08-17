#!/usr/bin/env python3

"""Download test fonts used by the FreeType regression test programs.  These
will be copied to $FREETYPE/tests/data/ by default."""

import argparse
import collections
import hashlib
import io
import os
import requests
import sys
import zipfile

from typing import Callable, List, Optional, Tuple

# The list of download items describing the font files to install.  Each
# download item is a dictionary with one of the following schemas:
#
# - File item:
#
#      file_url
#        Type: URL string.
#        Required: Yes.
#        Description: URL to download the file from.
#
#      install_name
#        Type: file name string
#        Required: No
#        Description: Installation name for the font file, only provided if
#          it must be different from the original URL's basename.
#
#      hex_digest
#        Type: hexadecimal string
#        Required: No
#        Description: Digest of the input font file.
#
# - Zip items:
#
#   These items correspond to one or more font files that are embedded in a
#   remote zip archive.  Each entry has the following fields:
#
#      zip_url
#        Type: URL string.
#        Required: Yes.
#        Description: URL to download the zip archive from.
#
#      zip_files
#        Type: List of file entries (see below)
#        Required: Yes
#        Description: A list of entries describing a single font file to be
#          extracted from the archive
#
# Apart from that, some schemas are used for dictionaries used inside
# download items:
#
# - File entries:
#
#   These are dictionaries describing a single font file to extract from an
#   archive.
#
#      filename
#        Type: file path string
#        Required: Yes
#        Description: Path of source file, relative to the archive's
#          top-level directory.
#
#      install_name
#        Type: file name string
#        Required: No
#        Description: Installation name for the font file; only provided if
#          it must be different from the original filename value.
#
#      hex_digest
#        Type: hexadecimal string
#        Required: No
#        Description: Digest of the input source file
#
_DOWNLOAD_ITEMS = [
    {
        "zip_url": "https://github.com/python-pillow/Pillow/files/6622147/As.I.Lay.Dying.zip",
        "zip_files": [
            {
                "filename": "As I Lay Dying.ttf",
                "install_name": "As.I.Lay.Dying.ttf",
                "hex_digest": "ef146bbc2673b387",
            },
        ],
    },
]


def digest_data(data: bytes):
    """Compute the digest of a given input byte string, which are the first
    8 bytes of its sha256 hash."""
    m = hashlib.sha256()
    m.update(data)
    return m.digest()[:8]


def check_existing(path: str, hex_digest: str):
    """Return True if |path| exists and matches |hex_digest|."""
    if not os.path.exists(path) or hex_digest is None:
        return False

    with open(path, "rb") as f:
        existing_content = f.read()

    return bytes.fromhex(hex_digest) == digest_data(existing_content)


def install_file(content: bytes, dest_path: str):
    """Write a byte string to a given destination file.

    Args:
      content: Input data, as a byte string
      dest_path: Installation path
    """
    parent_path = os.path.dirname(dest_path)
    if not os.path.exists(parent_path):
        os.makedirs(parent_path)

    with open(dest_path, "wb") as f:
        f.write(content)


def download_file(url: str, expected_digest: Optional[bytes] = None):
    """Download a file from a given URL.

    Args:
      url: Input URL
      expected_digest: Optional digest of the file
        as a byte string
    Returns:
      URL content as binary string.
    """
    r = requests.get(url, allow_redirects=True)
    content = r.content
    if expected_digest is not None:
        digest = digest_data(r.content)
        if digest != expected_digest:
            raise ValueError(
                "%s has invalid digest %s (expected %s)"
                % (url, digest.hex(), expected_digest.hex())
            )

    return content


def extract_file_from_zip_archive(
    archive: zipfile.ZipFile,
    archive_name: str,
    filepath: str,
    expected_digest: Optional[bytes] = None,
):
    """Extract a file from a given zipfile.ZipFile archive.

    Args:
      archive: Input ZipFile objec.
      archive_name: Archive name or URL, only used to generate a
        human-readable error message.

      filepath: Input filepath in archive.
      expected_digest: Optional digest for the file.
    Returns:
      A new File instance corresponding to the extract file.
    Raises:
      ValueError if expected_digest is not None and does not match the
      extracted file.
    """
    file = archive.open(filepath)
    if expected_digest is not None:
        digest = digest_data(archive.open(filepath).read())
        if digest != expected_digest:
            raise ValueError(
                "%s in zip archive at %s has invalid digest %s (expected %s)"
                % (filepath, archive_name, digest.hex(), expected_digest.hex())
            )
    return file.read()


def _get_and_install_file(
    install_path: str,
    hex_digest: Optional[str],
    force_download: bool,
    get_content: Callable[[], bytes],
) -> bool:
    if not force_download and hex_digest is not None \
      and os.path.exists(install_path):
        with open(install_path, "rb") as f:
            content: bytes = f.read()
        if bytes.fromhex(hex_digest) == digest_data(content):
            return False

    content = get_content()
    install_file(content, install_path)
    return True


def download_and_install_item(
    item: dict, install_dir: str, force_download: bool
) -> List[Tuple[str, bool]]:
    """Download and install one item.

    Args:
      item: Download item as a dictionary, see above for schema.
      install_dir: Installation directory.
      force_download: Set to True to force download and installation, even
        if the font file is already installed with the right content.

    Returns:
      A list of (install_name, status) tuples, where 'install_name' is the
      file's installation name under 'install_dir', and 'status' is a
      boolean that is True to indicate that the file was downloaded and
      installed, or False to indicate that the file is already installed
      with the right content.
    """
    if "file_url" in item:
        file_url = item["file_url"]
        install_name = item.get("install_name", os.path.basename(file_url))
        install_path = os.path.join(install_dir, install_name)
        hex_digest = item.get("hex_digest")

        def get_content():
            return download_file(file_url, hex_digest)

        status = _get_and_install_file(
            install_path, hex_digest, force_download, get_content
        )
        return [(install_name, status)]

    if "zip_url" in item:
        # One or more files from a zip archive.
        archive_url = item["zip_url"]
        archive = zipfile.ZipFile(io.BytesIO(download_file(archive_url)))

        result = []
        for f in item["zip_files"]:
            filename = f["filename"]
            install_name = f.get("install_name", filename)
            hex_digest = f.get("hex_digest")

            def get_content():
                return extract_file_from_zip_archive(
                    archive,
                    archive_url,
                    filename,
                    bytes.fromhex(hex_digest) if hex_digest else None,
                )

            status = _get_and_install_file(
                os.path.join(install_dir, install_name),
                hex_digest,
                force_download,
                get_content,
            )
            result.append((install_name, status))

        return result

    else:
        raise ValueError("Unknown download item schema: %s" % item)


def main():
    parser = argparse.ArgumentParser(description=__doc__)

    # Assume this script is under tests/scripts/ and tests/data/
    # is the default installation directory.
    install_dir = os.path.normpath(
        os.path.join(os.path.dirname(__file__), "..", "data")
    )

    parser.add_argument(
        "--force",
        action="store_true",
        default=False,
        help="Force download and installation of font files",
    )

    parser.add_argument(
        "--install-dir",
        default=install_dir,
        help="Specify installation directory [%s]" % install_dir,
    )

    args = parser.parse_args()

    for item in _DOWNLOAD_ITEMS:
        for install_name, status in download_and_install_item(
            item, args.install_dir, args.force
        ):
            print("%s %s" % (install_name,
                             "INSTALLED" if status else "UP-TO-DATE"))

    return 0


if __name__ == "__main__":
    sys.exit(main())

# EOF
