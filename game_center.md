# SubGame GuessHash Instructions

## Overview

[SubGame Center](https://gamecenter.subgame.org/)

First of all, if you have successfully set up GuessHash, you can see the following screen.

![](https://i.imgur.com/GkAq8oH.png)

After connecting to the [wallet](https://polkadot.js.org/extension/), you can see that there is currently only one game called GuessHash in SubGame.

![](https://i.imgur.com/1mfcxhw.png)

After you click to enter the game, you can view the game rules as follows:
1. The game uses SGP as a bargaining chip
2. The game odds are all 1:1
3. The game decides the winner by guessing the end of the hash as odd or double.
4. Hash is determined based on block generation.
5. The bookmaker can limit the total bet limit of all players of the authority.

![](https://i.imgur.com/4hPwi61.png)

## Get chips
1. Before the game, you need to exchange for in-game chips (SGP), as shown below.
2. Here we use the ```chip``` [module](https://github.com/SubGame-Network/subgame-network/blob/master/pallets/chips/src/lib.rs), through which the native currency SGB can be exchanged with the in-game chip SGP.

![](https://i.imgur.com/BtM20Bo.png)

## Bookmaker

1. Then we can start the game. In the game, we can act as "Bookmaker" & "Player". Let's take a look at the bookmaker's part first and click "create". You can see that the bookmaker can choose the upper limit of the bet for the entire game, and the amount of the upper limit of the bet will be pledged to the end of the game, depending on the result of the game to decide whether to get it back.
2. Here we use the ```game (guess-hash)``` [module](https://github.com/SubGame-Network/subgame-network/blob/master/pallets/gametemplates-guess-hash/src/lib.rs), through which openings can be created, bets and final results can be calculated.

![](https://i.imgur.com/uWSv1CM.png)


Then you can go into the room to see who is betting and some basic information, because you cannot bet on your own game if you are the bookmaker.

![](https://i.imgur.com/WsMRU1O.png)

When the result appears, there will be a pop-up prompt, you can also go to the history record to query.

![](https://i.imgur.com/nDQLxkb.png)

In addition, you can also click ![](https://i.imgur.com/FzB6YNa.png) to go to Polkscan to check.
Then check the details of the transaction

![](https://i.imgur.com/RORzfsm.png)

## Player

Now let's take a look at the role of the player. You can participate in the game by clicking "Quick Join" or directly clicking the room.

![](https://i.imgur.com/gCql9aS.png)


After entering the room, you can see the basic information. If you do not exceed the betting limit of this game, you can place a bet.

![](https://i.imgur.com/kZP91Mz.png)

When the result appears, there will be a pop-up prompt, you can also go to the history record to query.

![](https://i.imgur.com/ZIX3MJP.png)

In addition, you can also click ![](https://i.imgur.com/FzB6YNa.png) to go to Polkscan to check.
Then check the details of the transaction

![](https://i.imgur.com/RORzfsm.png)
