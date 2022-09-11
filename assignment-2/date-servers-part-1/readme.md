# Date servers - Part I

## Assignment

You should create two new servers (important that you don’t reuse the same two 
servers from this weeks’ other assignment since loose coupling is emphasized 
during this course).

Both will be the "date authority"

* implement a GET "/timestamp" endpoint that returns the current timestamp (ISO 8601).
 

The other server should eventually be able to request the timestamp and parse
it, but you don’t have to implement this this week. That follows next week. 

 

Note: When calling FastAPI use 127.0.0.1 instead of aliasing localhost. 

 

Bonus: If you are using Python as one of your programming languages you could 
deploy the server to [pythonanywhere](www.pythonanywhere.com). It’s free.
