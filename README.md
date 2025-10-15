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
| **SET**     | `SET R n`       | Coloca o valor n em R                                                             | `SET FUND1 10`           |
| **ADD**     | `ADD R1/n R2`   | Adiciona e guarda um valor em R2                                                  | `ADD 1 FUND1`         |
| **SUB**     | `SUB R1/n R2`   | Subtrai e guarda um valor em R2                                                   | `SUB 1 FUND2`         |
| **MULT**     | `MULT R1/n R2`   | Multipla R2 por um valor (inteiro) e guarda em si mesmo.         | `MULT 1 FUND1`         |
| **DIV**     | `DIV R1/n R2`   | Divide R2 por um valor (inteiro) e guarda em si mesmo.             | `DIV 1 FUND2`         |
| **GOTO**    | `GOTO label`    | Pula para a label                                                                 | `GOTO loop`           |
| **GOTOZ**   | `GOTOZ R/n label` | Pula para a label se R ou n for 0                                                      | `GOTOZ S1 loop`       |
| **PRINT**   | `PRINT R/n`       | Printa o valor atual do registrador                                               | `PRINT BALANCE`       |
| **PUSH**    | `PUSH R/n`      | Coloca um valor no stack                                                          | `PUSH 10`             |
| **POP**     | `POP R`         | Tira um valor do stack e coloca em R                                              | `POP FUND1`           |
| **CRASH**   | `CRASH`         | Para o programa                                                                   | `CRASH`               |
| **BUY**     | `BUY n`         | Compra n ações. Se `BALANCE` for menor do que `n * STOCKPRICE`, para o programa.  | `BUY 10`              | 
| **SELL**    | `SELL n`        | Vende n ações. Se `OWNED` for menor do que `n`, para o programa.                 | `SELL 1`              | 

# Men Lang

Men Lang é uma linguagem de programação pensada para se assimilar ao diálogo tradicional paulistano da Faria Lima, mais conhecido como "menzinho".

### EBNF 

```ebnf
program: 
    /* empty */
    | program statement
    ;

statement:
    declaration
    | print
    | increment
    | decrement
    | conditional
    | conditional_loop
    | ENDLINE
    ;

declaration:
    DECLARE IDENTIFIER ASSIGN expr ENDLINE
    ;

print:
    PRINT expr ENDLINE
    ;

increment:
    IDENTIFIER INC ENDLINE
    ;

decrement:
    IDENTIFIER DEC ENDLINE
    ;

conditional:
    IF bool_expr THEN noend_block { elseif_conditional } [else_conditional] ENDBLOCK
    ;

elseif_conditional:
    | IF ELSE bool_expr THEN noend_block
    ;

else_conditional:
    | ELSE THEN noend_block
    ;

conditional_loop:
    WHILE UNTIL bool_expr block
    ;

bool_expr:
    bool_term
    | bool_expr OR bool_term
    ;

bool_term:
    rel_expr
    | bool_term AND rel_expr
    ;

rel_expr:
    expr
    | expr bool_bin_op expr
    ;

bool_bin_op:
    EQUALS TO
    | GREATER THAN
    | LESSER THAN
    ;

expr:
    factor
    | expr bin_op factor
    | PLUS factor %prec UNARY_MINUS
    | MINUS factor %prec UNARY_MINUS
    | NOT factor
    ;

bin_op:
    PLUS | MINUS | DIVIDE | MULT
    ;

factor:
NUMBER
    | BOOLEAN
    | IDENTIFIER
    | LPAREN bool_expr RPAREN
    ;

block:
    noend_block ENDBLOCK
    ;

noend_block:
    statement
    | noend_block statement
    ;

BOOLEAN:
    "bizarro"
    | "certeza"
    | "depende"
    ;

IF: "se" ;
THEN: "ent";
ENDBLOCK: "ne";
WHILE: "grind";
UNTIL: "enquanto";
ELSE: "pah";
DECLARE: "literal" | "literalmente";
ASSIGN: "eh";
OR: "ou";
AND: "e";
NOT: "nao";
EQUALS: "identico";
TO: "a";
GREATER: "maior";
LESSER: "menor";
THAN: "que";
PLUS: "mais";
MINUS: "menos";
INC: "win";
DEC: "loss";
DIVIDE: "divide";
MULT: "vezes";

ENDLINE:
    "men" | "mano" | "meo" | "menzinho" | "meu" | "bro" | "vei" ;

PRINT: 
    "po" | "puts" | "puta" ;

IDENTIFIER:
    LETTER { LETTER | DIGIT };

NUMBER:
    DIGIT { DIGIT } ;

LETTER:
    [a-zA-Z_] ;

DIGIT:
    [0-9];


```

### Example 

```
literal a1 eh 2 vei
literalmente b eh 2 mais 3 man
literal c_2 eh bizarro men

po bizarro vei
puts a1 vezes b vei

grind enquanto a1 menor que 5 
    po a1 men
    a1 win men
ne
    
```

Output:
```
bizarro
10
2 
3 
4
```



