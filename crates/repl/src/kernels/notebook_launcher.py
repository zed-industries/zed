import sys

from ipykernel.kernelapp import IPKernelApp

app = IPKernelApp.instance()
app.initialize()


def enable_notebook_imports(info):
    app.shell.events.unregister("pre_run_cell", enable_notebook_imports)
    if "" not in sys.path:
        sys.path.append("")


app.shell.events.register("pre_run_cell", enable_notebook_imports)
app.start()
