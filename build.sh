#!/usr/bin/env bash
# --------------------------------------------
# runmen.sh
# Executa um programa MenLang completo:
#  1. Compila o arquivo .men usando o compilador 'comp'
#  2. Executa o arquivo gerado (.invm) na Investment VM ('vm')
#  3. Remove o arquivo de saída .invm
# --------------------------------------------

# interrompe se algo der errado
set -e

# Verifica se recebeu um argumento
if [ $# -lt 1 ]; then
    echo "Uso: $0 <arquivo.men>"
    exit 1
fi

# Nome do arquivo de entrada
SRC="$1"

# Verifica se o arquivo existe
if [ ! -f "$SRC" ]; then
    echo "Erro: arquivo '$SRC' não encontrado."
    exit 1
fi

# Define o nome base (sem extensão)
BASENAME=$(basename "$SRC" .men)
OUTFILE="out.invm"

# Compila o arquivo MenLang
echo "🔧 Compilando '$SRC'..."
./comp "$SRC"

# Confirma se a compilação gerou o arquivo
if [ ! -f "$OUTFILE" ]; then
    echo "❌ Erro: compilação não gerou '$OUTFILE'."
    exit 1
fi

# Executa o arquivo gerado na VM
echo "🚀 Executando '$OUTFILE' na Investment VM..."
./vm "$OUTFILE"

# Apaga o arquivo gerado
echo "🧹 Limpando arquivo temporário..."
rm -f "$OUTFILE"

echo "✅ Execução finalizada com sucesso."

