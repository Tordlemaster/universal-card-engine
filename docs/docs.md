# Overview

### States and Routines
UCE scripts are composed of a set of discrete game States, which control the rules of the game. Game data, like decks and players, are stored independently of the States. Every State has a name and a sequence of Routines, which defines the proceeding of the game when in that State. Any game action can be represented by a Routine, including but not limited to player turns, player choices, or verifying the validity of the game state.

### Decks
Every collection of cards, whether draw pile, discard pile, player's hand, etc., is represented by a Deck. In addition to their cards, Decks have parameters which determine which players are shown their contents. They have:

A stack parameter, which describes whether a Deck is a stack of cards where at most only the top card can be visible, or an array of cards which might all be visible,

A face up or down parameter, which if face down overrides other visibility parameters and makes the deck face down for all players,

A global visibility parameter, which if true makes the deck visible to all players regardless of other parameters,

A list of player names to whom the deck is visible,

And a list of teams to whom the deck is visible.

# Grammar
The following is simplified for readability, but should be descriptive enough to allow the reader to write their own scripts. For the actual grammar used by the parser, consult /src/grammar.lalrpop.

## States
### States: STATES { State+ }
The body of a UCE script begins with a STATES scope, in which the game's states are declared. The game will begin in the state named "SETUP".

### State: "\<state name>" { Routine+ }
The definition of a State consists of its name *in quotes and in all capital letters* followed by its Routines in an inner scope. Its sequence of Routines must end with a "STATE" routine, which switches to the next state. If ending the game is desired, the state to switch to should be the reserved name "_END".

## Variables
Run-time information about a game can be stored in variables. While custom variables can only be initialized when switching States through the "STATE" routine, several default variables are built-in. Variables can be read by wrapping the name of the variable in square brackets as part of a string, which when evaluated for use will be replaced by the value of the variable.

### [THISPLAYER]
This variable is available within FORPLAYER and FORPLAYERCOND routines. Its value is automatically set to the name of the player corresponding to the current iteration.

### [#] and [N]
This variable allows for the creation and manipulation of decks whose quantity is unlimited in the course of a game. For example, in Rummy cards from players' hands are dealt to create new decks called "melds", of which there can be any number. To allow for this, including "[#]" in a string will perform pattern-matching with all existing deck names which are equal to the provided string except for the "[#]", and evaluate to the value one more than the highest existing value. When evaluated within a Choice or "IF" Routine, the same value will be available to the Choice or Routine's Conditions. Including "[N]" in a string will prompt the user to supply the value to which it will be evaluated. This will also be passed to the Conditions of the enclosing Choice or "IF" Routine.

## Conditions and Choices
Choices allow for the user to select an action from a set. Conditions enforce the game rules on the user's choice.
There are three types of Conditions, listed below.

### NOT \<Condition>
The value will be the inverse of the value of the enclosed condition.

### DECKCOND "\<deck name>" { DeckCond+ }
The enclosed DeckConds will be evaluated on the deck with the specified name. The following are all possible DeckConds.

SUITS:SAME: Whether all the cards in the deck have the same suit
VALS:(SAME|CONS): Whether all the cards in the deck have the same value or consecutive values
LEN:(LESS|LEQ|EQ|GEQ|GREATER)(\<length>): Use the specified comparison operator to compare the deck's length to the enclosed length

### PLAYERCOND "\<player name>" { PlayerCond+ }
The enclosed PlayerConds will be evaluated on the player with the specified name. The following are all possible PlayerConds.

NAME("\<player name>"): Whether the player's name is equal to the enclosed name

SCORE:(LESS|LEQ|EQ|GEQ|GREATER)(\<score>): Use the specified comparison operator to compare the player's score to the enclosed score

### Choice: "\<message>" (PRE|POST|)COND AND|OR { Condition+ } { Routine+ }
The message is a description of the action performed by the choice, presented to the user when the set of choices is printed. PRECOND means the conditions will be evaluated before the routines are executed. POSTCOND means the conditions will be evaluated after the routines are executed, and they will be undone if the conditions are not met. The third argument determines whether the set of conditions will be evaluated together using conjunction or disjunction.

### Choice: "\<message>" NOCOND { Routine+ }
The same as the above, except the routines will be executed no matter what if the choice is selected.


## Routines
The following are all currently implemented Routines. More will be added as they are required to support additional card games.

### NEWDECK/NEWSDECK "\<deck name>" \<visibility params> [\<player names>] \<(A)rray|(S)tack (U)p/(D)own >
These routines add new decks to the game world. They require the deck's name in quotes, its visibility parameters, and an array in square brackets of the names of the players, in quotes, who should be shown the deck's contents.
The visibility parameters are represented by a sequence of three characters: A for array or S for stack, U for face up or D for face down, and finally 1 for globally visible and 0 otherwise. For more information on these parameters, see the "Overview" section.
NEWDECK creates an empty deck, while NEWSDECK creates a deck filled with the entire default 52-card deck, without jokers.

### DEALTOP "\<source deck name>" "\<destination deck name>" n
Deal the top n cards from the source deck to the top of the destination deck, one by one. Deck names must be in quotes.

### DEALRAND "\<source deck name>" "\<destination deck name>" n
Deal n cards from random positions in the source deck to the top of the destination deck, one by one. Deck names must be in quotes.

### DEALCHOICE "\<source deck name>" "\<destination deck name>" n(optional)
Prompt the user to select a subset of cards from the source deck to deal to the destination deck. If n is present, it represents the number of cards that must be chosen. If n is not present, the user may choose any number of cards.

### SCOREADD "\<deck name>" "\<player name>"
Evaluate the specified deck using a scoring function, and add the result to the score of the specified player. In the future, defining a custom scoring function will be supported, but right now the default scoring function returns the total of the values of the cards in the deck.

### PRINT "\<message>"
Print a message to the terminal. Intended to be used for giving a player custom prompts or additional information about the rules.

### FORPLAYER { Routine+ }
Runs a sequence of routines once for each player in the game. Enclosed routines have access to a [THISPLAYER] variable, which evaluates to the name of the player corresponding to the current iteration. This is intended to be used for player turns.

### FORPLAYERCOND { Condition+ } { Routine+ }
The same as FORPLAYER, except only iterates for players who satisfy the conditions. Enclosed conditionals also have access to the [THISPLAYER] variable.

### LOOP { Routine+ }
Runs its sequence of routines forever. This can only be broken out of by using the "STATE" routine.

### CHOOSE n(optional) { Choice+ }
Presents a set of choices to the user, each containing a sequence of routines and an optional set of conditions. For more information about Choices, see the "Conditions and Choices" section. If n is present, it represents the number of choices that must be made before moving on. If n is not present, the user may continue choosing actions as long as they wish, and the additional choice "End turn" will be presented, allowing them to move on.

### IF { Condition+ } { Routine+ }
If the set of conditions are satisfied, the sequence of routines will be executed.

### STATE "\<name of next state>" [\<initial variables>]
Switch states to the one with the specified name, with a set of initial variables. This can be used to keep track of game-specific information across states, such as winning players or teams. The set of variables should be written as an array in square brackets, and each entry should consist of the name of the new variable in quotes and without brackets, an equals sign, and its value in quotes.