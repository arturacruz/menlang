# Men Lang

### EBNF

```ebnf
<program>       ::= { <statement> }
<statement>     ::= {( <declaration> | <print> )}
<declaration>   ::= ( "literal" | "literalmente" ), <identifier>, "eh", <expr>, <endline>
<un-op>         ::= "mais" | "menos"
<bin-expr-op>   ::= <un-op>
<bin-term-op>   ::= "divido" | "vezes"
<bin-op>        ::= <bin-expr-op> | <bin-term-op>
<expr>          ::= <term> { <bin-expr-op> <term> }
<term>          ::= <factor> { <bin-term-op> <factor> }
<factor>        ::= <number> 
                  | <identifier>
                  | <un-op> <factor>
                  | "(" <expr> ")"
<print>         ::= ( "po" | "puts" | "puta" ), <expr>, <endline>
<endline>       ::= "men" | "mano" | "meo" | "menzinho" | "meu" | "bro" | "vei"
<endblock>      ::= "ne"
<identifier>    ::= <letter> {( <letter> | <digit> | "_" )}
<boolean>       ::= "bizarro" | "certeza" | "depende"
<number>        ::= [ <un-op> ] <digit> { <digit> }
<letter>        ::= "a".."z" | "A".."Z"
<digit>         ::= "0".."9"
```

### Example 

```
literal a eh 2 vei
literalmente b eh 2 mais 3 man
literal c_2 eh bizarro men

po a vezes b men
```
`output = 10`

