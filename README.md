# RPQ

## Description

RPQ is an RPG proof-of-concept web application.

## Reason for Name

It's an RPG, but instead of a G it's a Q. My first name is Quinten and I often like to replace G with Q as they look similar both in uppercase (G vs Q) and lowercase (g vs q).

## Anatomy of the Project

```
src/
    models.rs - Canonical models for this RPG
    db.rs - All database interactions, which are currently hand rolled csv interactions
    html/
        *.rs - Utilities to help with building the html pages to serve
        pages/
            *.rs - `page()` functions to build each page to serve
    http/
        route_map.rs - Map (match statement) of urls to the handlers for each route
        routes/
            *.rs - Route handlers for each url route
    main.rs - Main function that exposes this all on a web server
db/
    *.csv - Database csv files (one file per "table")
images/
    *.png - Images to serve
```

## Why Build it this Way?

Obviously it's pretty wild to request the entire page for each action,
and storing information like the client cursor information on the server.
This is a kind of proof-of-concept of how far I can take building an old-school web application,
just using HTML/HTTP to build pages to send to clients which render and then request more pages.

## Screenshot

![Screenshot of game web app](/screenshots/edit_mode_map_game.gif)
