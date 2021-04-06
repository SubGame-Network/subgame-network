# subgame Project
* Gaming chips(`pallet-chips`)：
It is one of the core modules of SubGame, which mainly includes chip purchase, exchange, and income distribution.
![](https://i.imgur.com/UKhJ00s.png)

* Game template library(`pallet-gametemplates`)：
It is an ever-growing library of game templates, and new games are continuously added through public chain upgrades.
![](https://i.imgur.com/o0nS8iu.png)



## The first game template
Template name： `pallet-gametemplates-guess-hash`

Design

Random factor: the hash string of the future block
Gameplay:
The player creates a new game instance and sets the parameters
Block number for guessing
Minimum and maximum bet amount
Match odds
Number of chips
All players can participate in the game and place bets before the guessed square appears
When the target block is generated, the game chips will be settled according to the game rules on the chain



## Test Case
Cucumber, a commonly used tool in BDD mode, supports three important purposes:
* It's can describe a clear and executable specification,  which is human-readable, and Cucumber can also be parsed.
* Automated testing can be carried out through Cucumber to verify whether the software meets the description of the specification.
* The software's functional behavior can be documented through specifications.


![](https://i.imgur.com/FLfi9Y8.png)


### test game(guess-hash) module

#### 【Scenario】Test the start round function
【Given】
```
A user has 100 chips
```
【When】
```
A user create game 
```
【Then】
```
Check chip balance = 0
Check chip pledge = 100

#Check if the game information is correct
Check that the starter is correct
Check that the betting blockchain is set correctly (current blockchain + next n blocks)
Check that the prize pool amount is correct
```

---
#### 【Scenario】Test the betting function

【Given】
```
A user has 100 chips
B user has 100 chips
A user have a new game, game index = 1
```
【When】
```
B user bet 100 chips/ bet number
```
【Then】
```
Check chip balance=0
Check chip pledge = 100

# Check the bet storage parameters
Check that the bettor is correct
Check that the betting game index is correct
Check that the bet amount is correct
Check betting game mode = odd number
```
---
#### 【Scenario】Test whether the reward distribution is correct
【Given】
```
A user has 500 chips
B user has 100 chips
C user has 100 chips
D user has 100 chips
A user have a new game, game index = 1, pool = 500
B bet single num, 100 chips
C bet single num, 100 chips
D bet double num, 100 chips
```
【When】
```
After reaching the lottery block
```
【Then】


when single is winner
```
Check A chip balance = 500-200 + 100
Check A chip pledge=0
Check B chip balance = 200
Check B chip pledge=0
Check C chip balance = 200
Check C chip pledge=0
Check D chip balance = 0
Check D chip pledge = 0
```

when double is winner
```
Check A chip balance = 500 + 200-100
Check A chip pledge=0
Check B chip balance = 0
Check B chip pledge=0
Check C chip balance = 0
Check C chip pledge=0
Check D chip balance = 200
Check D chip pledge = 0

```

### Test chip module

#### 【Scenario】Buy chips 1


【Given】
```
First purchase of chips
```
【When】
```
Buy 1000 chips
```
【Then】
```
Check chip balance=1000
```


---

#### 【Scenario】Buy chips 2
【Given】
```
Buy chips again
```
【When】
```
Buy another 1,000 chips again
```
【Then】
```
Check chip balance=2000
```
---
#### 【Scenario】The balance is not enough to buy chips
【Given】
```
F user not have chips
```
【When】
【Then】
```
Buy chips && return Error MoneyNotEnough
```
---
#### 【Scenario】redemption
【Given】
```
A user has 1000 chips
```
【When】
```
redemption
```
【Then】
```
Check chip balance=0
```
---
#### 【Scenario】redemption failed (no chips have been purchased)
【Given】
```
F user no chips have been purchased
```
【When】
```
redemption
```
【Then】
```
return Error ChipsIsNotExist
```
---
#### 【Scenario】Redemption failed (insufficient chips)
【Given】
```
F user has 10 chips
```
【When】
```
redemption
```
【Then】
```
 return Error ChipsIsNotExist
```