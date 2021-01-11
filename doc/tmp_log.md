A reservation should be paid within certain time limit after it is created. A lazy check is preferable. To realize it
the item system needs to store a reference to the user or the reference so it can check when the reservation was made.

Or, we can add one more state: Reserved (Unpaid) to the items. 

Either way, it is a bit complicated. I decided to use a universal event system. When a reservation is made, an alarm
event is set in the event system. The system periodically checks about timed events and send messages once they are
triggered.