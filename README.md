# Elephant Carpacio 
from Alistar Cockburn workshop (https://docs.google.com/document/d/1TCuuu-8Mm14oxsOnlk8DqfZAA1cvtYu9WGv67Yj_sSk/pub)

## Overview
Releasing software is actually releasing decisions. Every delivered decision is actually a choice that can be wrong. So what we release in production can contains a set of wrong decisions. 
Furthermore a decision can depend on other decisions, so a failure on one early decision impact the subsequent ones. This mean that rework an early decision is more difficult if not impossible to do.
Thereby a failure in one early decision can consequently challenge the entire delivery. 

How to fix? reducing the amount of decision per release, and organize the release like a business test on the delivered decision. If some assumption was wrong you can still rework and adjust, before going on.

### Postulates
#### Taking a lot of decisions without any feedback increase the risk of failure.

#### Reducing the amount of decisions in a delivery reduce the risk, and enable an early feedback


## Learning outcomes
* How to slice large applications into 1-day to 1-week requests, **business perspective**.
* How to slice application requests into 15-30 minute work slices, **programming perspective**.


## Assignment 
Accept inputs from the user:
* **Quantity**: a number of items,
* **Price**: a price for the item,
* **State**: a 2-letter state code

Compute the price of the order, giving a discount based on order value (not number of items), adding state tax based on the state and discounted order value.

| **Order value** | **Discount rate** | | **State** | **Tax rate** |
|-----------------|-------------------|-|-----------|--------------|
| $ 1,000         | 3%                | | UT        | 6.85%        |
| $ 5,000         | 5 %               | | NV        | 8.00%        |
| $ 7,000         | 7%                | | TX        | 6.25%        |
| $ 10,000        | 10%               | | AL        | 4.00%        |
| $ 50,000        | 15%               | | CA        | 10.00%       |


## Slices
| #   | Inputs                                          | Output                                                                          | Value                                                                       |
|-----|-------------------------------------------------|---------------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| 1.  | no input                                        | Output: Hello world                                                             | setup the project                                                           |
| 2.  | _Quantity_ and _Price_                          | Output: echo inputs                                                             | having a walking skeleton                                                   |
| 3.  | _Quantity_ and _Price_                          | Output: order value with no discount and fixed UT tax                           | Possibility to enable a test in UT                                          |
| 4.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and tax based on UT tax + echo _State_     | verify that the enter _State_ is correct                                    |
| 5.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and Tax based on UT tax                    | Automatic calculation of UT tax from the mapping of _State_ with _tax_rate_ |
| 6.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and Tax based on UT and NV _State_         | Go live on NV                                                               |
| 7.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and Tax based on UT, NV, TX                | Go live on TX                                                               |
| 8.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and Tax based on UT, NV, TX, AL            | Go live on AL                                                               |
| 9.  | _Quantity_, _Price_ and _State_                 | Output: order value with no discount and tax based on UT, NV, TX, AL, CA        | Go live on CA                                                               |
| 10. | _Quantity_, _Price_ and _State_                 | Output: order value with discount on $1,000 and tax based on UT, NV, TX, AL, CA | Possibility to put a discount over $1,000                                   |
| 11. | _Quantity_, _Price_ and _State_                 | Output: order value with discount on $5,000 and tax on all _State_              | Possibility to put discount over $5,000                                     |
| 12. | _Quantity_, _Price_ and _State_                 | Output: order value with discount on $7,000 and tax on all _State_              | Possibility to put discount over $7,000                                     |
| 13. | _Quantity_, _Price_ and _State_                 | Output: order value with discount on $10,000 and tax on all _State_             | Possibility to put discount over $10,000                                    |
| 14. | _Quantity_, _Price_ and _State_                 | Output: order value with discount on $10,000 and tax on all _State_             | Possibility to put discount over $50,000                                    |
| 15. | _Quantity_, _Price_ and _State_ from fancy cli  | Output: order value with discount on $10,000 and tax on all _State_             | User insert input through interface                                         |

