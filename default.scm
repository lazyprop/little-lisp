(define (map f lst)
  (if (null? lst)
      null
      (cons (f (car lst)) (map f (cdr lst)))))

(define (add1 x) (+ x 1))
(define sample-list (cons 1 (cons 2 (cons 3 null))))

(define (fib n)
  (if (<= n 1)
      n
      (+ (fib (- n 1))
         (fib (- n 2)))))

(define (and l r)
  (if l
      r
      false))

(define (or l r)
  (if l
      true
      r))

(define (cc amount kind)
  (cond ((eq? amount 0) 1)
        ((or (< amount 0) (eq? kind 0)) 0)
        (else (+ (cc amount (- kind 1))
                 (cc (- amount (denom kind)) kind)))))

(define (denom n)
  (cond ((eq? n 1) 1)
        ((eq? n 2) 5)
        ((eq? n 3) 10)
        ((eq? n 4) 25)
        ((eq? n 5) 50)))

(define (count-change amount) (cc amount 5))
