import os
from dotenv import load_dotenv

configs = load_dotenv()

print("are configs loaded? ",configs)
print("My secret is: ", os.environ.get("SECRET"))