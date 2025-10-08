# Investment VM

A InVM (ou Investment VM) será baseada na lógica de investimentos: alta, baixa, venda e compra de ações.

### Registradores 

Todos são registradores i32.

- `FUND1`: Contém o dinheiro investido na StartUp 1.
- `FUND2`: Contém o dinheiro investido na StartUp 2.

### Sensores

#### Mutáveis

Estes sensores mudam a cada operação para um valor aleátorio simulado.

- `SHARES`: Quantas shares estão no mercado.
- `STOCKPRICE`: Valor de uma *share* atualmente no mercado.
- `REPUTATION`: Determina a probabilidade de uma nova compra / venda, e muda aleatóriamente.

Se o `STOCKPRICE` está baixo e `REPUTATION` alta, haverá uma maior chance de compra.
Se o `STOCKPRICE` está alto a `REPUTATION` baixa, haverá uma maior chance de venda.

#### State 

Estes sensores só descrevem a situação atual.

- `MARKETVAL`: Valor total de dinheiro no mercado. Efetivamente, `SHARES` * `STOCKPRICE`.
- `EQUITY`: Valor de todas as *shares* do usuário. Efetivamente, `OWNED` * `STOCKPRICE`.
- `OWNED`: Contém quantas *shares* são possuídas pelo usuário no mercado. Começa em 0.
- `BALANCE`: Contém o capital atual. Começa em `10000` e aumenta em `100` toda operação.

#### Instruções

| Instrução   | Sintaxe         | Descrição                                                                         | Exemplo               |
|-------------|-----------------|-----------------------------------------------------------------------------------|-----------------------|
| **SET**     | `SET R n`       | Coloca o valor n em R                                                             | `SET S1 10`           |
| **ADD**     | `ADD R1/n R2`   | Adiciona e guarda um valor em R2                                                  | `ADD 1 FUND1`         |
| **SUB**     | `SUB R1/n R2`   | Subtrai e guarda um valor em R2                                                   | `SUB 1 FUND2`         |
| **GOTO**    | `GOTO label`    | Pula para a label                                                                 | `GOTO loop`           |
| **GOTOZ**   | `GOTOZ R label` | Pula para a label se R for 0                                                      | `GOTOZ S1 loop`       |
| **PRINT**   | `PRINT R`       | Printa o valor atual do registrador                                               | `PRINT BALANCE`       |
| **PUSH**    | `PUSH R/n`      | Coloca um valor no stack                                                          | `PUSH 10`             |
| **POP**     | `POP R`         | Tira um valor do stack e coloca em R                                              | `POP FUND1`           |
| **CRASH**   | `CRASH`         | Para o programa                                                                   | `CRASH`               |
| **BUY**     | `BUY n`         | Compra n ações. Se `BALANCE` for menor do que `n * STOCKPRICE`, para o programa.  | `BUY 10`              | 
| **SELL**    | `SELL n`        | Vende n ações. Se `OWNED` for menor do que `n`, para o programa.                 | `SELL 1`              | 

# Men Lang

Men Lang é uma linguagem de programação pensada para se assimilar ao diálogo tradicional paulistano da Faria Lima, mais conhecido como "menzinho".

### EBNF 

```ebnf
program        = { statement };
statement      = {( declaration
               | print
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



