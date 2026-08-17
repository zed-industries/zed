#!/bin/sh

RUSTCFLAGS=-g cargo bench --bench bench -- --profile-time 15
