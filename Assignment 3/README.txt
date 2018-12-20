Please fill out each TODO item in the header but change nothing else,
particularly nothing before the colon ":" on each line!

===================== HEADER ========================

Student #1, Name: Seungyup Lee
Student #1, ugrad.cs.ubc.ca Login: c7a1b
Student #1, Student Number: 10177160

Student #2, Name: Andrew Choi
Student #2, ugrad.cs.ubc.ca Login: h7h0b
Student #2, Student Number: 19792150

Team name (for fun!): Tired and Confused

Acknowledgment that you understand and have followed the course's collaboration policy (READ IT at
http://www.ugrad.cs.ubc.ca/~cs311/current/_syllabus.php#Collaboration_Policy):

Signed: Seungyup Lee, Andrew Choi

===================== LOGISTICS =====================

Please fill in each of the following:

Approximate hours on the project: 50+

Acknowledgment of assistance (per the collab policy!):

For teams, rough breakdown of work: 50/50, we both created and fixed and created... recurs

====================== THEORY =======================

1. In the description, we hinted that some features were similar to
existing features. In particular, "defquery" and "infer" share a
similarity with another pair of features, particularly in the way they
handle environments and delayed evaluation.  Explain which features
these are and what the similarity is.

They both use thunkV to delay their expression. They both use it as a place-holder/rest of list for later interp, but what's interesting about it is that it keeps environments, which rest-list cannot - infer uses this to construct a list, emulating rest-list, while defquery does not.

2. Suppose instead of (interp), we had a function with the following signature:
;;interp-with-sampler: (SOMETHING) RSE? -> RSE-Value?
(define (interp-with-sampler sample-from-dist expr) ...)

That is, suppose we allowed someone calling this
function to interpret an expression, using
their own random number generator for "sample"
expressions.

What signature would we use in place of (SOMETHING)?
That is, what should the inputs and outputs of the
"sample-from-dist" function be?

"distV?", would be our guess, it the user is using their own sample of numbers, distV would be a sampled list of values, which interp would recurse into for actual values. This one's tricky.

3. In assignment 2, to add two distributions,
we looked at all combinations of values from the two.
This feature was removed in this assignment.
Given distributions D1 and D2, how could we
approximate {+ D1 D2} in our language,
using sample, defquery, and infer?

We would not use sample, as D1 and D2 are already given. We would possibly assign the first distribution to work with into infer, and the other into defquery, to delay, until interp-ing. After that, create a local lambda map to add each element respectively.

======================= BONUS =======================

If you attempted any bonuses, please note it here and describe how you approached them.

TODO