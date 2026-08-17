$env:Path += ";$(Split-Path -Path (Get-Childitem –Path $Env:JAVA_HOME -Filter jvm.dll -Recurse) -Parent)"

cargo test --features=invocation
