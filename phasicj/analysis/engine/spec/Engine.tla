------------------------- MODULE Engine ---------------------------------------

EXTENDS Integers, Sequences

CONSTANT Queries
CONSTANT Rules
CONSTANT Facts

VARIABLE openQueries
VARIABLE log

TypeOK == /\ openQueries \in SUBSET Queries
          /\ log \in Seq(Facts)
          
Init == /\ Len(log) = 0
        /\ Len(openQueries) = 0

===============================================================================