import datetime
import sys
import requests

def main():
    version = sys.argv[1]
    version = version.removeprefix("v")
    project_id = sys.argv[2]
    account_username = sys.argv[3]
    account_secret = sys.argv[4]
    
    current_datetime = datetime.datetime.now(datetime.timezone.utc) 
    current_datetime = current_datetime.strftime("%Y-%m-%d %H:%M:%S")
    
    url = f"https://mixpanel.com/api/app/projects/{project_id}/annotations"
    
    payload = {
        "date": current_datetime,
        "description": version
    }
    
    response = requests.post(
        url, 
        auth=(account_username, account_secret), 
        json=payload
    )


if __name__ == "__main__":
    main()