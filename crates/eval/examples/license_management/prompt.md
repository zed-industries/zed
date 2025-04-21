I need to switch our license stuff from the old .reuse/dep5 file to the new REUSE.toml format. basically same info, just different format. here's what's in the old file:

project name: abap-cheat-sheets
contact: daniel reger's email
repo link
that long SAP API disclaimer
copyright: SAP + contributors, 2022
license: Apache-2.0
need to:

delete the old .reuse/dep5 file
make a new REUSE.toml with:
same project info (name, contact, repo)
same exact API disclaimer text
SPDX-style copyright & license fields
apply to all files (** glob) with aggregate precedence
not changing any actual license terms, just updating the format. can you give me the exact REUSE.toml file we need?
