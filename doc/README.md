# OOAD Lab 3: Flight Reservation System

I have tried twice on the project, and both failed. By the time I've already missed the deadline. I started too high,
yet my engineering skills were way too insufficient to support the goals. It's painful to admit it but, I have to start
really small, before I could handle a project that is of industrial-product quality.

It really feels bad to cast away a project with numerous analysis & design docs, and a code base of tens of KBs. But I'm
pretty sure that if I don't hone my engineering skills steadily, with very-small baby-steps, I don't get what I really
want ever.

Alas.

## Analysis

*The flight-reservation system* (FRS) is a simplified system.

The intrinsic features of a reservation system requires that it stores some states. The states are:

- *User information*. Since reservations are involved with money, and refunding is required, we need to record that "who
  did what reservations". Reservations are no longer stored as an independent state as in previous tries, since it makes
  little sense for a reservation to live with no owner, and we can easily get around the limitation by providing a void
  owner.

- *Reservable-Items information*. "Reservable Item" is a general term that refers to anything that users are able to
  reserve through this system. The most trivial one should be the passenger tickets. Introducing the abstraction allows
  us to support other items such as luggage-checking tickets, and pick-up tickets etc.

Correspondingly, the core subsystems are:

- *User system*. It records user information such as reservations a user made, and authenticates users. Reservations
  stored in the system does not own the items. Instead, they refer to items in the Item system by an Item Id.

- *Item system*. It records available items. When reserving, the system checks if the items are available, and handles
  extra work related with items instead of the users.

### Glossary

The [glossary](analysis/glossary.md) records terms of the system. However, with the most detailed design documents in the source
code, the glossary is of little use now. Turned out that it more acted as a memorandum.

### Use cases

You can quickly grasp a whole picture of the system in the [use case graph](./use caes graph.puml).

***CLAIM***:

Due to the limitation of markdown, alternative sequences such as `1a.` are replaced with unordered sequences under the
number. I wanted to integrate the main scenario and extensions, but to respect the rule I just replicated the main
scenario in the extensions instead of merging them.

Some steps do not have strict order. For these cases I may use unordered list (i.e. `-` symbols instead of numbers).

> *Login as a use case*
>
> The three tests of the value of a use case claims that *login* is not a good use case to be started in the analysis
> phase. It is done in the elaboration phase.

1. [Reserve Tickets](analysis/uc1.md)

### System Sequence Graph (SSD)

The SSDs are to record external events that the system will respond to. These are:
1. Events from User.
2. Time events, such as time limit exceeded for unpaid reservations.
3. External errors and exceptions. Since in this lab no external cooperators exist, such events are impossible.

We can see that system events are fewer than anticipated.

## Design

### UC1: Reserve Tickets



