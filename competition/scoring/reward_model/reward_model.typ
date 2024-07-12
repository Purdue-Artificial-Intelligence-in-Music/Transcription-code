A game exists with the following rules:
- You must guess if a weighted coin is H or T.
- You must provide the certainty of your guess.
- You are rewarded based upon a combination of your certainty and if your were correct or incorrect.

Devise a scoring system such that players are incentivized to accurately indicate their certainty in their answer.

= Definitions
/ $R_p$: reward if correct
/ $R_n$: reward if incorrect
/ $p$: player's accuracy
/ $c$: player's reported accuracy
/ $E=R_p p + R_n (1-p)$: expectation value of player's guess
/ $M (p) $: optimal strategy for determining $c$ which maximizes expectation


= Naive
$ R_p = c, R_n = -c $

Then $ E=c p - c (1-p) = c(p-(1-p))= c (2p - 1) = 2p c - c $

#figure(image("naive.png"), caption: [#text(blue, [Expectation]) value for $text(#green, c), text(#red, p)$])
// #grid(columns: 3)[
// #figure(image("naive_pact_0.51.png"), caption: [#text(purple, [Expectation]), #text(red, [positive reward]), #text(blue, [negative reward])])][
// #figure(image("naive_pact_0.49.png"), caption: [#text(purple, [Expectation]), #text(red, [positive reward]), #text(blue, [negative reward])])]


and so $ M (p) = cases(c=0 &"if" p<0.5\ c=1 &"if" p>0.5) $

This naive reward system encourages accurately knowing $p$ but lying about it's precise value.

*Is there a better reward metric such that the optimal strategy is to not lie i.e. $p=M(p)$?*
