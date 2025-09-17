# Men Lang

Men Lang é uma linguagem pensada para se assimilar ao diálogo tradicional paulistano da Faria Lima, mais conhecido como "menzinho".

### EBNF

```ebnf
<program>           ::= { <statement> }
<statement>         ::= {( <declaration> 
                      | <print> 
                      | <conditional> 
                      | <conditional-loop> 
                      | <increment> )}
<declaration>       ::= ( "literal" | "literalmente" ), <identifier>, "eh", <expr>, <endline>
<conditional>       ::= "se", <expr>, <no-endblock> {("se" <else-conditional>)} [ <else-conditional> ] "ne"
<conditional-loop>  ::= "grind", "ate", <expr>, <block>
<else-conditional>  ::= "pah", <no-endblock>
<un-op>             ::= "mais" | "menos"
<increment>         ::= <identifier> (("win" | "W") | ("loss" | "L")) <endline>
<bool-un-op>        ::= "não"
<bool-bin-op>       ::= "e" | "ou" | "eh"
<bin-op>            ::= <un-op> | "divide" | "vezes" | <bool-bin-op> | <bool-un-op>
<expr>              ::= <factor> { <bin-op> <factor> }
<factor>            ::= <number> 
                      | <identifier>
                      | <un-op> <factor>
                      | "(" <expr> ")"
<block>             ::= <no-endblock> "ne"
<noend-block>       ::= <statement> { <statement> }
<print>             ::= ( "po" | "puts" | "puta" ), <expr>, <endline>
<endline>           ::= "men" | "mano" | "meo" | "menzinho" | "meu" | "bro" | "vei"
<endblock>          ::= "ne"
<identifier>        ::= <letter> {( <letter> | <digit> | "_" )}
<boolean>           ::= "bizarro" | "certeza" | "depende"
<number>            ::= [ <un-op> ] <digit> { <digit> }
<letter>            ::= "a".."z" | "A".."Z"
<digit>             ::= "0".."9"
```

### Example 

```
literal a eh 2 vei
literalmente b eh 2 mais 3 man
literal c_2 eh bizarro men

grind ate a eh 5 
    se a eh 3 
        po a men 
    se pah a eh 4 
        po a divide 4 men 
    pah 
        po a vezes 2 men
    ne

    a win men
ne
            
```

Output:
```
4
3
1
```

# Investment VM

A InVM será baseada em investimentos, alta, baixa, venda e compra de ações.

