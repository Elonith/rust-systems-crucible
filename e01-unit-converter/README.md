# E01: CONVERT CLI TOOL

This is a simple command line tool that is used to convert values from one unit to another. It's used in the following format: `convert <number> <from_unit> <to_unit>`, for example `convert 72 F C` means "convert 72°F(Fahrenheit) to °C(Celsius)" and `convert 5 mi Km` means "convert 5 miles to Kilometers". The result is printed with 2 dp(decimal places)--for example, 22.22 will be printed instead of 22.2222222222.

## How to interpret Error messages

There are two types of error messages:

- _Wrong usage_--_exit code 1_: This could mean:

1. Units from different families: for example-- `convert 5 km kg`
2. The number isn't a valid number: for example-- `convert five mi cm`
3. Wrong number of arguments: for example-- `convert 7 inc `

- _Unknown unit_--exit code 2: for example-- `convert 88 mi parsecs`

## What can be converted

Four families of units:

- **Temperature**: Celsius, Fahrenheit, Kelvin
- **Length**: metric and imperial units, such as meters, kilometers, miles, feet and inches.
- **Mass**: for example kilograms, grams, pounds and ounces.
- **Data size**: for example bytes, kilobytes, megabytes and gigabytes. I use 1024 for kilobytes conversion
