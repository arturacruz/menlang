# Men Lang

Men Lang é uma linguagem pensada para se assimilar ao diálogo tradicional paulistano da Faria Lima, mais conhecido como "menzinho".

### EBNF

```ebnf
program        = { statement };
statement      = {( declaration
               | print
               | conditional
               | conditional_loop
               | increment )};
declaration    = ( "literal" | "literalmente" ), identifier, "eh", expr, endline;
un_op          = "mais" | "menos";
increment      = identifier, (("win" | "W") | ("loss" | "L")), endline ;
bin_op         = un_op | "divide" | "vezes" ;
expr           = factor, { bin_op, factor };
factor         = number | identifier | (un_op, factor) | ("(", expr, ")") ;
block          = noend_block, "ne" ;
noend_block    = statement, { statement };
print          = ( "po" | "puts" | "puta" ), expr, endline;
endline        = "men" | "mano" | "meo" | "menzinho" | "meu" | "bro" | "vei" ;
identifier     = letter, {( letter | digit | "_" )} ;
boolean        = "bizarro" | "certeza" | "depende" ;
number         = digit, { digit };
letter         = "a" | "..." | "z" | "A" | "..." | "Z" ;
digit          = "0" | "1" | "..." | "9" ;
```

### Example 

```
literal a eh 2 vei
literalmente b eh 2 mais 3 man
literal c_2 eh bizarro men

po bizarro vei
puts a vezes b vei
            
```

Output:
```
bizarro
10
```

# Investment VM

A InVM será baseada em investimentos, alta, baixa, venda e compra de ações.

