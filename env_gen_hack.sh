#!/bin/sh

#dear code reviewer/interviewer, please run this script to generate the .env file so that you can run this project on your local machine

#as a HACK store the creds in base64 format and generate the env file on demand
creds=("U1FMX0RCX0NPTk5fU1RSPSJTZXJ2ZXI9dGNwOm1oYWhleHB0cy5kYXRhYmFzZS53aW5kb3dzLm5ldCwxNDMzO0luaXRpYWwgQ2F0YWxvZz1kYlNwaWNlQUlBc3NnbjtQZXJzaXN0IFNlY3VyaXR5IEluZm89RmFsc2U7VXNlciBJRD11bmFtO1Bhc3N3b3JkPVNwMWN5RGF0YTIwMjQ7TXVsdGlwbGVBY3RpdmVSZXN1bHRTZXRzPUZhbHNlO0VuY3J5cHQ9VHJ1ZTtUcnVzdFNlcnZlckNlcnRpZmljYXRlPUZhbHNlO0Nvbm5lY3Rpb24gVGltZW91dD0zMDsiCg=="
"REJSU19BQ0NFU1NfVE9LRU49ZGFwaTY0MzI5NmU4NGMxNzZhMjgyZDRjZjFlYWMwZTZiYWUwCg=="
"T1BFTkFJX0FQSV9LRVk9InNrLXByb2otUHBFZDFtSXVKeHpvOHJuWjdyMERjUDlaQWZ4NzJoNHBTQVJ1eURvbXZkQW1ERTdUR00za2FVOEFVZll2TlF6RUIzZFBtMC1HZDBUM0JsYmtGSlNhWnBSNkpXbm9SYkhUaDFPUENTcExoMFJuNW9FX3hmamQ4Z0ZuZm9kQzlhOWFOYlB1NHZoNGliME1CeC12S0NmVDVSU2thRFFBIgo=")

# Clear the .env file or create it if it doesn't exist
: > .env

# Iterate through the array and decode each Base64 string
for encoded in "${creds[@]}"; do
    # Decode the Base64 string
    decoded=$(echo "$encoded" | base64 --decode)
    # Append the decoded string to .env file
    echo "$decoded" >> .env
done
