import json
import os
import sys
from pathlib import Path
from string import Template


def kernel_env(prefix):
    from jupyter_client.kernelspec import KernelSpec
    from traitlets import TraitError

    kernel_dir = prefix / "share" / "jupyter" / "kernels" / "python3"
    try:
        spec = KernelSpec.from_resource_dir(str(kernel_dir))
        if spec.language.lower() != "python":
            return {}
        return {
            (key.upper() if os.name == "nt" else key): Template(value).safe_substitute(os.environ)
            for key, value in spec.env.items()
        }
    except (OSError, ValueError, TypeError, TraitError):
        return {}


if __name__ == "__main__":
    import ipykernel  # The exit status also tells Zed whether it needs to install ipykernel.

    print(json.dumps(kernel_env(Path(sys.prefix))))
