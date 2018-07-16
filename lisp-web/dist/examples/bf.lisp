(println "BF Interpreter")
(func eval_bf (source) (
    (let gen_code "")
    (let ptr 0)
    (let mem_size 1024)
    (let mem (fill mem_size 0))

    (func update_val (op) (
        (let val (op (nth mem ptr) 1))
        (if (eq val 256) (set val 0))
        (if (eq val -1) (set val 255))
        (set mem (nth mem ptr val))
    ))

    (func update_ptr (op) (
        (let val (op ptr 1))
        (if (eq val mem_size) (set val 0))
        (if (eq val -1) (set val (- mem_size 1)))
        (set ptr val)
    ))

    (map source (lambda (token)(
        (set gen_code (concat gen_code
        (if (= token "[") ("(loop (!(=(nth mem ptr) 0)) (") (""))
        (if (= token "]") ("))") (""))
        (if (= token ">") ("(update_ptr +)") (""))
        (if (= token "<") ("(update_ptr -)") (""))
        (if (= token "+") ("(update_val +)") (""))
        (if (= token "-") ("(update_val -)") (""))
        (if (= token ".") ("(print (char (nth mem ptr)))") (""))
        (if (= token ",") ("(read)") (""))
        ))
        (nil)
    )))

    (println "Source BF:")
    (println source)
    (println "")
    (println "Generated Code:")
    (println gen_code)
    (println "")
    (println "BF Output:")
    (eval gen_code)
    (println "")
))

(eval_bf "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.")
