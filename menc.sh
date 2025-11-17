#!/usr/bin/env bash

if [ $# -lt 1 ]; then
    echo "Uso: $0 <arquivo.men>"
    exit 1
fi

SRC="$1"

if [ ! -f "$SRC" ]; then
    echo "Erro: arquivo '$SRC' não encontrado."
    exit 1
fi

BASENAME=$(basename "$SRC" .men)
OUTFILE="out.invm"

echo "Compilando '$SRC'..."

./menc "$SRC"

./vm "$OUTFILE"

rm -f "$OUTFILE"
