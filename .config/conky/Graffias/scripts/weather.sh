#!/bin/bash

# Closeboc73
# This script is to get weather data from openweathermap.com in the form of a json file
# so that conky will still display the weather when offline even though it doesn't up to date

# you can use this or replace with yours
api_key=29c7c3f06ff45f58f6a2e409c2fb2d22
# get your city id at https://openweathermap.org/find and replace
city_id=3439389

url="api.openweathermap.org/data/2.5/weather?id=${city_id}&appid=${api_key}&cnt=5&units=metric&lang=en"
curl ${url} -s -o ~/.cache/weather.json
