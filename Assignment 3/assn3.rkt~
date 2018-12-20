#lang plai
(require "./assn3-library.rkt")
;; CS311 2018W1 A3

(print-only-errors)

(define-syntax-rule (test/rejected e1) (test (rejected? e1) #true))

;; ==========================================================
;;                     EBNF & DEFINE-TYPES
;; ==========================================================


;; RSE = Rejection Sampling Expression
;;
;; <RSE> ::= <number>
;;     | {distribution <RSE>+}
;;     | {uniform <number> <number>}
;;     | {sample <RSE>}
;;     | {assert-in <RSE> <RSE>}
;;     | {defquery RSE}
;;     | {infer <RSE> <RSE>}
;;     | {<Binop> <RSE> <RSE>}
;;     | {ifelse <RSE> <RSE> <RSE>}
;;     | {with {<id> <RSE>} <RSE>}
;;     | {fun {<id>} <RSE>}
;;     | {<RSE> <RSE> } ;; function calls, any time the first expression is not a reserved symbol
;;     | <id>
;;
;; <Binop> ::= + | - | * | / | < | > | = | <= | >=
;;
;; Note: <id> cannot be one of these reserved words:
;;       with distribution sample assert-in defquery infer uniform ifelse
;;       fun + - * / < > = <= >=
;;
;; Note: the expression {uniform a b} evaluates to a discrete uniform 
;;       distribution from a to b (including a but EXCLUDING b).
;;       Unlike assignment 2, we disallow empty distributions. Therefore,
;;       The case a >= b is not allowed and must result in an error!
;;
;;       For example, {uniform 3 6} means the same as the range notation
;;       [3, 6) and includes the numbers 3, 4, and 5.

(define-type RSE
  [distribution (values (and/c (listof RSE?) (not/c empty?)))] ;;Distributions must not be empty
  [num (n number?)]
  [id (name symbol?)]
  [binop (op procedure?) (lhs RSE?) (rhs RSE?)]
  [with (name symbol?) (named-expr RSE?) (body RSE?)]
  [ifelse (cond RSE?) (t RSE?) (e RSE?)]
  [fun (param symbol?) (body RSE?)]
  [app (function RSE?) (arg RSE?)]
  [sample (distExp RSE?)]
  [defquery (body RSE?)]
  [infer (num-times RSE?) (query RSE?)]
  [assert-in (cond RSE?) (body RSE?)])

;; Environments store values, instead of substitutions
(define-type Env
  [mtEnv]
  [anEnv (name symbol?) (value RSE-Value?) (env Env?)])

;; Interpreting an expression returns a Value
(define-type RSE-Value
  [numV (n number?)]
  [distV (values (and/c (listof RSE-Value?) (not/c empty?)))] ;;Distributions must not be empty
  [rejected] ;; The value that a failing assert-in interprets to
  [thunkV (body RSE?) (env Env?)] ;;A thunk is just a closure with no parameter
  ;;Useful for defquery
  [closureV (param symbol?)  ;;Closures wrap an unevaluated function body with its parameter and environment
            (body RSE?)
            (env Env?)])

;;canonical-dist: distV? -> distV?
;;Turn a distribution into a canonical form
;;Makes testing work nicely
;;Only works on (non-empty) distributions of numV
(define (canonical-dist d)
  (distV (map numV (canonical-list (map numV-n (distV-values d))))))

;;canonical-list: (listof number?) -> (listof number?)
;;Helper for above. Precondition: l is non-empty.
(define (canonical-list l)
  (if (list? l)
      (local ([define elements
                (remove-duplicates l)] ; all the unique elements
              [define elements-counts
                (map
                 (lambda (e)
                   (count (lambda (x) (equal? x e)) l))
                 elements)]  ;the count of those unique elements
              [define elements-gcd
                (apply gcd elements-counts)] ; gcd of those counts
              [define new-lists
                (map
                 (lambda (num source-count)
                   (make-list (/ source-count elements-gcd) num))
                 elements elements-counts)])
        (sort (flatten new-lists) <))
      (error "expected a list")))

;; ==========================================================
;;                           PARSE
;; ==========================================================

;; lookup-op : symbol -> (or procedure false)
(define (lookup-op op)
  (match op
    ['+ +]
    ['* *]
    ['- -]
    ['< <]
    ['> >]
    ['<= <=]
    ['>= >=]
    ['= =]
    ['/ (lambda (lhs rhs)
          (if (= 0 rhs)
              (error "division by 0")
              (/ lhs rhs)))]
    [_ #f]))

;; any->boolean : any -> boolean
(define (any->boolean x)
  (if x
      #t
      #f))

;; op-exists? : symbol -> boolean
(define (op-exists? op) (any->boolean (lookup-op op)))
(test (op-exists? '*) #t)
(test (op-exists? 'with) #f)

(define *reserved-keywords*
  '(with distribution uniform sample assert-in defquery infer ifelse fun))

;; reserved? : symbol -> boolean
(define (reserved? word)
  (any->boolean
   (or
    (member word *reserved-keywords*)
    (lookup-op word))))

(test (reserved? '*) #t)
(test (reserved? 'with) #t)
(test (reserved? 'foo) #f)

;; parse : s-exp -> RSE
;; Consumes an s-expression and generates the corresponding RSE
(define (parse sexp)
  (local [(define valid-id? (and/c symbol? (not/c reserved?)))]
    (match sexp
      [(? number?)
       (num sexp)]
      [(list (and op (? symbol?) (? op-exists?)) lhs rhs)
       (binop (lookup-op op) (parse lhs) (parse rhs))]
      [(list 'ifelse condn then elsee)
       (ifelse (parse condn) (parse then) (parse elsee))]
      [(list 'with (list (and (? valid-id?) id) value) body) (with id (parse value) (parse body))]
      [(list 'assert-in constr body) (assert-in  (parse constr) (parse body))]
      [(list 'uniform (? number? a) (? number? b))
       (if (>= a b)
           (error "Tried to build a uniform distribution that is empty")
           (distribution (map num (range a b))))]
      [(list 'distribution values ...)
       (if (empty? values)
           (error "Cannot create empty distribution")
           (distribution (map parse values)))]
      [(list 'infer numruns query) (infer (parse numruns) (parse query))]
      [(list 'defquery query) (defquery (parse query))]
      [(list 'sample expr) (sample (parse expr))]
      [(? valid-id?) (id sexp)]
      [(list 'fun (list (? valid-id? param)) body) (fun param (parse body))]
      [(cons (and word (? reserved?)) _)(error 'parse "Misused reserved word ~a in: ~a" word sexp)]
      [(list f arg) (app (parse f) (parse arg))]
      [_ (error 'parse "Unable to recognize expr: ~a" sexp)]
      )))

;; ==========================================================
;;                           INTERP
;; ==========================================================

;;wrapResult : (or boolean number) -> RSE-Value
;;Helper function for turning the result of an operation into a numV
(define (wrapResult res)
  (cond
    [(boolean? res) (if res (numV 1) (numV 0))]
    [(number? res) (numV res)]
    [else (error "Binary operations should produce numbers or booleans")]
    )
  )

;; lookup : symbol Env -> RSE-Value
;; lookup looks up a variable name in a env
(define (lookup id env)
  (type-case Env env
    [mtEnv () (error "free variable")]
    [anEnv (name value anotherEnv) (if (symbol=? id name)
                                       value
                                       (lookup id anotherEnv))]))

;; interp : RSE -> RSE-Value
;; This procedure interprets the given RSE and produces a result 
;; in the form of a RSE-Value (either a closureV, thunkV, or numV),
;; using a specifically seeded random number generator (so we can test your result)
(define (interp expr)
  (local ([define sample-from-dist (get-default-sampler)]
          ;;Curried helper function that you might find useful
          ;;Feel free to not use this
          [define (interp-env env) (lambda (e) (interp-helper e env))]

          ;; interp-helper : RSE Env -> RSE-Value
          ;; interp-helper is the core of interp; you'll want to recurse using this
          [define (interp-helper expr env)
            (type-case RSE expr
              [id (name) (lookup name env)]
              [num (n) (numV n)]
              [binop (op lhs rhs)
                     (type-case RSE-Value (interp-helper lhs env)
                       [numV (n1)
                             (type-case RSE-Value (interp-helper rhs env)
                               [numV (n2) (wrapResult (op n1 n2))]
                               [rejected () (rejected)]
                               [else (error "non-numerical value in binop rhs")])]
                       [rejected () (rejected)]
                       [else (error "non-numerical value in binop lhs")])]
              [ifelse (cond conseq altern)
                      (type-case RSE-Value (interp-helper cond env)
                        [numV (n) (if (not (= n 0))
                                      (interp-helper conseq env)
                                      (interp-helper altern env))]
                        [rejected () (rejected)]
                        [else (error "non-boolean value in ifelse test")])]
              [fun (param body) (closureV param body env)]
              [app (f arg)
                   (error "TODO FIX ME")]
              [with (name named-expr body)
                    (interp-helper
                     (app (fun name body) named-expr)
                     env)]
              [distribution (elems) 
                            (error "TODO FIX ME")]
              [sample (e)
                      (local [(define v (interp-helper e env))]
                        (type-case RSE-Value v
                          [rejected () (rejected)]
                          [distV (values)
                                 (if
                                  (empty? values)
                                  (error "Cannot sample empty distribution")
                                  (sample-from-dist values))]
                          [else (error "Can only sample distributions")]))]
              [defquery (body) (error "TODO FIX ME")]
              [infer (n query)
                     (error "TODO FIX ME")]
              [assert-in (constr body)
                         (error "TODO FIX ME")])])
    ;; start with an empty env
    (interp-helper expr (mtEnv))))

;;Helper to make tests a bit shorter
;;Parse and run a program, assuming it returns a distV of numV values
;;run : sexp -> RSE-Value?
(define (run pgm)
  (local [(define result (interp (parse pgm)))]
    (if (distV? result)
        (canonical-dist result)
        result)))



;;;;;;;;;;;;; A3 tests ;;;;;;;;;;;;;;;;

;;;;;;; NOTE ;;;;;;
;;The results of these test cases rely heavily
;;on the random-number generator we have configured.
;;In a real program, you would get (slightly) different answers
;;every time you ran sample or inference.
;;But, that's really hard to test!
;;So we've configured the random number generator with a fixed seed, and you
;;should get the same results that we do.
;;Don't alter these tests, though you can write your own.

;;Environment tests
;;Do we get an error accessing an unbound variable?
(test/exn (run '{with {x 3} {+ x y}}) "")

;;Make sure we're using strict evaluation
(test/exn (run '{with {x y} 3}) "")

;;Same with functions
(test/exn (run '{{fun x 3} y}) "")

;;sampling constant distributions should always give the same results
(test
 (run
  '{infer 5 {defquery
              {with {d {uniform 3 4}}
                    {with {x {sample d}}
                          {+ x x}}}}})
 (distV (map numV '(6))))

;;sampling independent events should be independent
;;Probability states that as the number of trials increases, the distribution will approach
;;a 50-50 split.
;;We run it 6 times, since that gives us a 50-50 split,
;;but that's just luck, due to our RNG.

(test
 (run
  '{with {d {uniform 0 2}}
         {infer 6 {defquery      
                    {with {x {sample d}}
                          {with {y {sample d}}
                                y}}}}})
 (distV (map numV '(0 1))))




;;If we constrain that a coin-flip is heads,
;;then we should get a constant distribution
(test
 (run
  '{infer 10 {defquery
               {with {d {uniform 0 2}}
                     {with {x {sample d}}
                           {assert-in {< x 1}
                                      x}}}}})
 (distV (map numV '(0))))

;;If we roll a 6-sided dice,
;;and constrain that result is 1-3
;;distribution should roughly match rolling 3-sided dice
(test
 (run
  '{with {d {uniform 1 7}}
         {infer 10 {defquery      
                     {with {x {sample d}}
                           {assert-in {< x 4}
                                      x}}}}})
 (distV (map numV '(1 2 3))))


;;If we roll two 6-sided dice,
;;and constrain that the sum adds up to 6
;;what is the distribution on the first roll?
(test
 (run
  '{with {d {uniform 1 7}}
         {infer 1000 {defquery      
                       {with {x {sample d}}
                             {with {y {sample d}}
                                   {assert-in {= 6 {+ x y}}
                                              x}}}}}})
 (distV (map numV '(  1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
                        2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
                        3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3
                        4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4
                        5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5 5))))

;;Sample can return different things each time
(test
 (run '{with {d {uniform 1 51}}
             {with {f {fun {x} {sample d}}}
                   {- {f 0} {f 0}}}})
 (numV 41))


;;Make sure query not evaluated too soon
(test
 (run '{with {q1 {defquery {+ x 1} }}
             {with {q2 {defquery {+ 3 3}}}
                   {infer 2 q2}}}
      )
 (distV (map numV '(6)))
 )

;;Make sure we throw an error when give sample a bad value
(test
 (run `{sample {distribution 1 1 1}}) (numV 1))
(test/exn
 (run `{sample 3}) "")

;; Test some distribution cases.
(test
 (run `{distribution {+ 3 4} 5}) (distV (map numV '(5 7))))
(test/exn
 (run `{distribution {
                      3 4} 5}) "")

;;Error propagation tests
;;If, when we evaluate a sub-expression, we get rejected
;;the result should be rejected
;;except for infer, which filters out rejected
(test/rejected
 (interp
  (parse
   '{+ 1 {assert-in {> 1 1} 1}})))

(test
 (rejected? (interp
             (parse
              '{fun {x} {+ 1 {assert-in {> 1 1} x}}})))
 false)

;;Functions should delay evaluation
(test/rejected
 (interp
  (parse
   '{{fun {x} x} {assert-in {> 1 1} 1}})))

(test/rejected
 (interp
  (parse
   '{ifelse {assert-in {> 1 1} {> 2 3}}  5 6})))

(test
 (interp
  (parse
   '{assert-in {> 2 1}  0})) (numV 0))

(test/exn (interp (parse '{/ 1 {sample {distribution 0}}})) "")

;;parser tests
(test/exn (parse '{uniform 5 5}) "")
(test (parse '{uniform 1 4}) (distribution (map num '(1 2 3))))
(test (parse '{distribution {+ 1 2} 0})
      (distribution (list (binop + (num 1) (num 2)) (num 0))))
(test (parse 'f) (id 'f))
(test/exn (parse 'fun) "")
(test (parse '{fun (x) (+ x x)})
      (fun 'x (binop + (id 'x) (id 'x))))
(test (parse '{f x})
      (app (id 'f) (id 'x)))
(test (parse '(sample d))
      (sample (id 'd)))
(test (parse '(sample (uniform 1 2)))
      (sample (distribution (list (num 1)))))
(test (parse '(defquery q))
      (defquery (id 'q)))
(test (parse '(defquery
                (sample (uniform 0 1))))
      (defquery 
        (sample (distribution (list (num 0))))))
(test (parse '(infer (* 5 5)
                     (defquery (sample (uniform 0 1)))))
      (infer (binop * (num 5) (num 5))
             (defquery 
        (sample (distribution (list (num 0)))))))
(test (parse '(infer n q))
      (infer (id 'n) (id 'q)))
(test (parse '(* 10000 0))
      (binop * (num 10000) (num 0)))
(test (parse '(ifelse (+ 1 2) f (fun (x) x)))
      (ifelse (binop + (num 1) (num 2))
              (id 'f)
              (fun 'x (id 'x))))
(test (parse '(with (x 10) y))
      (with 'x (num 10) (id 'y)))
(test (parse '(fun (f) (f f)))
      (fun 'f (app (id 'f) (id 'f))))