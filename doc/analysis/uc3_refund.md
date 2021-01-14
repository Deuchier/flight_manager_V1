# UC3: Refund

## Level

user-goal

## Primary Actor

User

## Interests

- User:
    - Get back proper amount of money from refunded reservations.
    - Money goes back to its src.

## Preconditions

- User logged in.

## Postconditions

- Proper amount of money (see [technical specifications][t2]) is returned to the source.

## Main Success Scenario

1. User starts a refunding for a reservation.
2. [s2]User selects a reason for the refunding.
3. System checks the reason.
4. System performs the refunding.
5. System get informed of the refunding success.
6. System updates the state of the reservation.
7. System informs User of the success.
8. [s8]System closes the transaction.

## Extensions

1. User can enter with different approaches. （用户可以以不同的姿势进入这个用例 XD）
    - *through an independent entry*.
        1. System enlists all Users' reservations that are able to be refunded.
        2. User chooses a reservation.
    - *through the interface of a reservation*.
        1. System goes directly into [step 2][s2] in the Main Success Scenario.
2. User can select the following reasons:
    - *flight overdue*.
        1. System checks if the flight is overdue.
    - *I don't want it*.
        1. System checks if requirements are satisfied (see [technical specs][t1]).
3. System checks the reason.
    1. System finds that the reason is not valid. For the following scenarios:
        - *flight overdue*.
            1. System found that the flight is not overdue.
        - *I don't want it*.
            1. See [technical specs][t1].
    2. System informs User of the failure and give reasons for the failures.
    3. System goes to [step 8][s8].
4. ...
5. Refund failed.
    1. System informs User of the failure.
    2. System logs the anomaly.
    3. System terminates the refunding, the reservation stays effective.

## Technical Specifications

- [t1]Reservations that can be refunded must satisfy:
    1. has been paid, but not withdrawn.
    2. the flight is due no sooner than 2 hours later.

- [t2]For different refunding reasons, different portions (might exceed 100%) of the paid money will be returned. For
  example, if the plane were overdue more than 2 hours, User can get 120% percent of their refund. Note that the
  refunding strategies are likely to vary.
    1. *flight overdue*: 120%.
    2. *I don't want it*: 80%.
    
