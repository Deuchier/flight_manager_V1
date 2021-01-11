# UC1: Reserve Tickets

## Level

user-goal

## Primary Actor

User

## Interests

- User:
    - Minimal efforts of reservation.

## Preconditions

- User logged in.
- Normally, User should also view the flights, or other *item categories* before they actually do the reservation.

## Postconditions

- User reserved the ticket (Unpaid)

## Main Success Scenario

1. User starts an reservation.
2. System creates a blank reservation and show it to User.
3. User does an action.
4. System responds to the action.<br>
   *loop 3-5 until no more action needed*.
5. System presents a summary of the reservation to User.
6. User confirms.
7. System stores the reservation.
8. User pays within time limit (see *Special Requirements*).
9. System acknowledges the payment.

## Extensions

- At any time, when errors occur:
    - System records the error.
    - System informs User of the error.

1. ...
2. ...
3. The actions are:
    - *Add an item to the reservation*.
        1. User selects the item.
        2. System checks if the item is available.
            - Item available.
                1. System adds the item into the reservation list.
            - Item occupied or expired.
                1. System reports the anomaly.
    - *Remove an item from the reservation*.
        1. User selects an item.
        2. System checks if the item is in the list.
            - Item found.
                1. System removes the item from the list.
            - Item not found.
                1. System reports the anomaly.
    - *View the summary of the reservation*.
        1. System presents a summary page.
        2. User views as they like.
        3. User asks to go back.
        4. System returns to step 3.
4. ...
5. ...
6. User may also decline the reservation. In this case:
    1. System discards all information about the reservation, returning all items in the list.
    2. Reservation aborted.
7. ...
8. User may decline to pay, or may have not paid after the time limit. In this case:
    1. System does as in *Extension step 6*.

## Special Requirements

- User should pay within 5 minutes. After the limit, the reservation will be aborted.
- Since the project is only a simulation, we should let the payment succeed under any circumstances. I.e. the tickets
  are free!
- It's okay to have little UI.

## Notes

In *Extensions*, situations such as "item not found in the reservation list" are considered anomaly, for in normal cases
the user should only see what is valid. I.e. they can only issue *remove* commands on items in the list, and only issue
*reserve* commands on items available.

## Open issues

Manipulation of reservations should always be verified with user info. For simplicity concerns, I omitted the
requirement.