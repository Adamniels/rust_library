#!/bin/bash


GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

# Kör tarpaulin och få ren output
raw_output=$(cargo tarpaulin --out Html 2>&1 | \
  grep -Ev '^\x1B\[2m.*(INFO|DEBUG)' | \
  sed 's/\x1B\[[0-9;]*m//g' | grep '^test ')

# Extrahera unika suites
suites=$(echo "$raw_output" | sed -n 's/^test tests::\([^:]*\)::.*$/\1/p' | sort | uniq)

# Gå igenom varje suite och skriv ut resultaten
for suite in $suites; do
  echo -e "\n🔹 Test Suite: ${suite}"

  # Filtrera rader för denna suite
  echo "$raw_output" | grep "tests::$suite::" | while read -r line; do
    if [[ "$line" == *"... ok" ]]; then
      echo -e "  ${GREEN}${line}${NC}"
    elif [[ "$line" == *"... FAILED" ]]; then
      echo -e "  ${RED}${line}${NC}"
    else
      echo "  $line"
    fi
  done
done

# Visa summering
summary=$(cargo tarpaulin --out Html 2>&1 | grep 'test result\|coverage')
echo -e "\n📊 ${summary}"
echo -e "\033[0;32m> Running coverage with tarpaulin...\033[0m"

# # Kör tarpaulin och filtrera output
# cargo tarpaulin --out Html 2>&1 | \
#   grep -Ev '^\x1B\[2m.*(INFO|DEBUG)' | \
#   sed 's/\x1B\[[0-9;]*m//g'
