#lang racket
;; Library for Assignment 3.
;; Do not change anything in this file! Otherwise, your assignment might
;; not run correctly on the server!
(provide get-default-sampler
         plot-from-dist)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; DO NOT TOUCH THIS CODE
;; Doing so will make *all our tests fail*
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(define (get-default-sampler)
  (local ([define rng (vector->pseudo-random-generator
                       (vector 1062772744 4224666480 3040273072 1729335656 1332042050 2620454108))])
    (lambda (outcomes)
      (list-ref outcomes (random (length outcomes) rng)))
    ))

(local
  ([define s (get-default-sampler)]
   [define l '(1 2 3 4 5 6)]
   [define result (list (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l) (s l))])
  (if
   (equal? result '(6 1 1 3 3 5 6 2 2 4 4 4 3 2 4 6))
   "Random generator performing properly. Please do not mess with the generator code in the library file, or our tests will fail."
   (error 'get-default-sampler "Your random generator failed the sanity test with result ~v. Please notify the instructors immediately!" result)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Thanks to Sam Chow for this code!
;; Helper to visualize distributions
;; plot-from-dist: number? distV? -> (void)
;; takes the given number of random samples from the given distribution
;; and graphs the results
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(require plot/no-gui)

(define (plot-from-dist iterations loN)
  (local ([define rand-from (get-default-sampler)]
          [define all-results (build-list iterations (lambda (x) (rand-from loN)))]
          [define map-to-hash (foldl
                               (lambda (curr hash-acc)
                                 (if (hash-has-key? hash-acc curr)
                                     (hash-set hash-acc curr (+ 1 (hash-ref hash-acc curr)))
                                     (hash-set hash-acc curr 1)
                                     ))
                               (make-immutable-hash)
                               all-results)]
          [define sorted-vals (sort (hash->list map-to-hash) < #:key car)]
          [define (map-pairs-to-list x) (list (car x) (cdr x))]
          [define result-list (map map-pairs-to-list sorted-vals)])
    (plot-pict
     (discrete-histogram result-list))))