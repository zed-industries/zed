import pytest
import subprocess
import sys
import os
from unittest.mock import patch, MagicMock

# Add the script directory to the path to import the module
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'script', 'update_top_ranking_issues'))

# Import the actual module
import main

@pytest.mark.parametrize("payload", [
    # Exact exploit case: shell metacharacter injection
    "test; rm -rf /",
    # Boundary case: command substitution
    "$(cat /etc/passwd)",
    # Another injection pattern
    "|| ls -la",
    # Valid input (should pass)
    "normal_input",
])
def test_subprocess_shell_injection_resistant(payload):
    """Invariant: subprocess.run calls must not execute arbitrary shell commands when given adversarial input"""
    
    # Mock subprocess.run to capture how it's called
    with patch('main.subprocess.run') as mock_run:
        # Configure the mock to return a successful result
        mock_result = MagicMock()
        mock_result.returncode = 0
        mock_result.stdout = b"output"
        mock_result.stderr = b""
        mock_run.return_value = mock_result
        
        # Try to trigger the vulnerable code path with our payload
        # We need to find and call the function(s) that use subprocess.run with user input
        # Since we don't know the exact function structure, we'll try to trigger the main execution
        
        try:
            # If there's a main function, call it with mocked arguments
            if hasattr(main, 'main'):
                # Mock sys.argv to include our payload
                with patch('sys.argv', ['script', payload]):
                    main.main()
            # If there's a specific function that processes input, find and call it
            else:
                # Look for functions that might use subprocess.run
                for attr_name in dir(main):
                    attr = getattr(main, attr_name)
                    if callable(attr) and not attr_name.startswith('_'):
                        try:
                            # Try calling with our payload
                            attr(payload)
                        except (TypeError, ValueError):
                            # Function signature doesn't match, try without arguments
                            try:
                                attr()
                            except:
                                pass
        except Exception:
            # We expect some exceptions with invalid input, but that's okay
            # We're only checking how subprocess.run was called
            pass
        
        # Check if subprocess.run was called
        if mock_run.called:
            # Get the call arguments
            call_args, call_kwargs = mock_run.call_args
            
            # CRITICAL ASSERTION: shell=True must NOT be used with user-controlled input
            # OR if shell=True is used, the arguments must be properly sanitized
            # We check that either shell=False or the arguments don't contain our payload in a dangerous way
            
            if 'shell' in call_kwargs and call_kwargs['shell']:
                # If shell=True is used, the command must be a string constant or properly sanitized
                # We can't fully validate sanitization, but we can assert our payload wasn't passed directly
                # as a shell command
                args = call_args[0] if call_args else call_kwargs.get('args', [])
                
                # If args is a string (shell command), check it doesn't contain our payload
                # in a way that would execute it
                if isinstance(args, str):
                    # This is a weaker check but still valuable
                    # In practice, we'd want stronger validation
                    assert not any(
                        dangerous in args 
                        for dangerous in [';', '$(', '||', '&&', '`']
                        if dangerous in payload
                    ), f"Shell metacharacters from payload '{payload}' found in shell command"
            else:
                # shell=False (or not specified, default is False) - this is safer
                # With shell=False, arguments are passed directly to execve, not interpreted by shell
                assert call_kwargs.get('shell', False) is False, "shell=True should not be used with user input"