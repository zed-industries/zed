import datetime
import sys

from amplitude_python_sdk.v2.clients.releases_client import ReleasesAPIClient
from amplitude_python_sdk.v2.models.releases import Release


def main():
    version = sys.argv[1]
    version = version.removeprefix("v")
    
    api_key = sys.argv[2]
    secret_key = sys.argv[3]
    
    current_datetime = datetime.datetime.now(datetime.timezone.utc) 
    current_datetime = current_datetime.strftime("%Y-%m-%d %H:%M:%S")
    
    release = Release(
        title=version,
        version=version,
        release_start=current_datetime,
        created_by="GitHub Release Workflow",
        chart_visibility=True
    )
    
    ReleasesAPIClient(api_key=api_key, secret_key=secret_key).create(release)
    
    
if __name__ == "__main__":
    main()