(println "Welcome to the REPL!!")
(println "Try typing `debug_scopes` and pressing enter to see what functions are available.")
(let result nil)
// We use `yield_loop` here in order to pass execution back the the browser after each iteration to allow for event and UI processing
(yield_loop
 ('(println "result := " (set result (eval (readln))))))
