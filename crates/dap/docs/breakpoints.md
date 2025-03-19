# Overview

The active `Project` is responsible for maintain opened and closed breakpoints
as well as serializing breakpoints to save. At a high level project serializes
the positions of breakpoints that don't belong to any active buffers and handles
converting breakpoints from serializing to active whenever a buffer is opened/closed.

`Project` also handles sending all relevant breakpoint information to debug adapter's
during debugging or when starting a debugger.
