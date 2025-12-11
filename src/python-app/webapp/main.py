import json
from os.path import dirname, abspath, join
from fastapi import FastAPI
from fastapi.responses import RedirectResponse
from fastapi import HTTPException


current_dir = dirname(abspath(__file__))
historical_data = join(current_dir, "weather.json")

app = FastAPI()


# load historical json data and serialize it:
with open(historical_data, "r") as f:
    data = json.load(f)

@app.get('/')
def root():
    """
    Allows to open the API documentation in the browser directly instead of
    requiring to open the /docs path.
    """
    return RedirectResponse(url='/docs', status_code=302)


@app.get('/countries')
def countries():
    return list(data.keys())


@app.get('/countries/{country}/{city}/{month}')
def monthly_average(country: str, city: str, month: str):
    # Validate input and return 404 if not found
    if country not in data:
        raise HTTPException(status_code=404, detail="Country not found")
    if city not in data[country]:
        raise HTTPException(status_code=404, detail="City not found")
    if month not in data[country][city]:
        raise HTTPException(status_code=404, detail="Month not found")
    return data[country][city][month]
# Note: If the month is missing in the URL, FastAPI will return a 404 automatically.