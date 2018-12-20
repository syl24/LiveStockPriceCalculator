Please fill out each TODO item in the header.

Please don't reformat the HEADER area of this file, and change nothing except the TODOs, particularly nothing before the colon ":" on each line! 

We reserve the right to do (minor) point deductions for misformatted READMEs. 

===================== HEADER ========================

Student #1, Name: Seungyup Lee
Student #1, ugrad.cs.ubc.ca Login: c7a1b
Student #1, Student Number: 10177160

Student #2, Name: Andrew Choi
Student #2, ugrad.cs.ubc.ca Login: h7h0b
Student #2, Student Number: 19792150

Team name (for fun!): BearsRus

Acknowledgment that you understand and have followed the course's collaboration policy (READ IT at
http://www.ugrad.cs.ubc.ca/~cs311/current/_syllabus.php#Collaboration_Policy):

Signed: Seungyup Lee, Andrew Choi

===================== LOGISTICS =====================

Please fill in each of the following:

Acknowledgment of assistance (per the collab policy!): None

For teams, rough breakdown of work: We worked together through one machine, discussed how each implementation would be made and divided work to debug and create tests on ideas that we had.

====================== THEORY =======================

1. Give a simple test case for your interpreter that ensures it uses eager evaluation semantics. (Your test case may not rely on the amount of time it takes the interpreter to run.)

(with (x y) (with (y 1) x))
Would produce error based on eager evaluation!

2. As mentioned in the assignment overview, there are many ways to describe equivalent distributions. For example, (distribution '(1 1 2 2 3 3)), (distribution '(2 1 3 1 3 2)), and (distribution '(1 2 3)) are all the same (they have a 1/3 chance of a 1 or 2 or 3). We might want to be able to transform all of these to a minimal and canonical form. Minimal here means a distribution value contains as few numbers as possible. Canonical means any two equivalent distribution values transform to the same canonical distribution value. 

Describe the steps you would take to transform a distribution to a minimal, canonical form and how you could use such a transformation to check for equality.

Sort: rearrange the distribution in a sorted order to make the next steps easier and faster
Remove duplicates: iterate through the list, for each element remove everything between itself and the next non-duplicate element
After these steps the distribution will be in its minimal, canonical form. 
Any equivalent distribution will look identical after these steps only needs (equals? dist1 dist2) to check for equality.

3. Just for the following question, assume that the "observe-that" doesn't exist in our language and that distributions are never empty. Suppose we modify the evaluation of distributions in the following way: When a distribution is interpreted, a single number is immediately sampled from it (with the appropriate probability of it appearing). This single value is then used in the remainder of the program's execution. This is similar to the Arithmetic Expression (AE) Language from class, but with random starting values. 

Answer these two questions: 
a) Could your program output a final value that didn't appear in the original interpreter's distribution output? 

No, it would have the same output, since the question is asking for eager evaluation of what our parser-interpreter is asking for.

b) How would the results of multiple runs of this new interpreter relate to the the distribution that the original interpreter would have produced?

It would compute faster in terms of speed, as distribution is handled already without the need to recurse to the parser to get a distribution number via compop. The results of the multiple run should be about the same as long as the input's were the same.


======================= BONUS =======================

If you attempted any bonuses, please note it here and describe how you approached them.

TODO
