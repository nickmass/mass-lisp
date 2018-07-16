(func repl () (
    (let continue true)
    (let result nil)
    (let input nil)
    (loop continue ((set input (readln)) (println "result := " (if (= (trim input) "quit") (set continue false) (set result (eval input))))))
))

(repl)
