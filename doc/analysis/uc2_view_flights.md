# UC2: View Flights

## Level

user-goal

## Primary Actor

User

## Interests

- User:
    - view clear flights info
    - convenient filtering, querying and intelligent-suggestions.

## Preconditions

- User logged in.

## Postconditions

- User viewed the flights (items)

## Main Success Scenario

1. User queries about flights.
2. System processes the query.
3. System presents the result.

## Extensions

1. Users do the query...
    - *By departure place & destination*.
    - *They can also specify filters, sorting strategies, and so on*.
2. Some exceptions may occur. If so, System should report the error to the user and terminate the query.
    - *System can't find the places*.
    - *No flights satisfying the requirements exist*.
3. ...

## Open Issues

- Support queries of transferring flights.