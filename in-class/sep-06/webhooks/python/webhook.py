import requests
import json
import datetime

webhook_url = 'http://localhost:8080/webhook'
webhook_url_teacher = 'http://f729-94-18-243-162.ngrok.io/webhook'

data = {
    "description": "This is some data",
    "timestamp": datetime.datetime.now().isoformat()
}

result = requests.post(
    webhook_url_teacher,
    json=json.dumps(data),
    headers={"Content-Type": "application/json"}
    )

print(result.text)
