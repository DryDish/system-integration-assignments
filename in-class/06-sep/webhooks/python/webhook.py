import requests
import json
import datetime

webhook_url = 'http://localhost:8080/webhook'


data = {
    "description": "This is some data",
    "timestamp": datetime.datetime.now().isoformat()
}

result = requests.post(
    webhook_url,
    json=json.dumps(data),
    headers={"Content-Type": "application/json"}
    )

print(result.text)
