#!/bin/bash

echo -e "\033[0;32m> Kör kodtäckning med Tarpaulin...\033[0m"
echo -e "\033[0;34m> Running tests...\033[0m"

# Kör test och spara all output
output=$(cargo tarpaulin --out Html -- --nocapture 2>&1)

# Filtrera bort tarpaulin-loggar och kompilering
cleaned=$(echo "$output" | awk '
  /Compiling / { next }
  /Finished `test` profile/ { next }
  { print }
')

# Extrahera testresultat för gruppering
echo "$cleaned" | grep '^test ' | while read -r line; do
  if echo "$line" | grep -q 'queue_test'; then
    echo -e "\033[1;36m🔹 Queue Tests\033[0m"
    break
  fi
done

echo "$cleaned" | grep '^test tests::queue_test' | sed 's/test tests::queue_test::/- /'

if echo "$cleaned" | grep -q 'hashmap_test'; then
  echo ""
  echo -e "\033[1;36m🔹 Hashmap Tests\033[0m"
  echo "$cleaned" | grep '^test tests::hashmap_test' | sed 's/test tests::hashmap_test::/- /'
fi

# Visa slutresultat och fel
echo ""
echo "$cleaned" | grep '^test result:'
echo ""
echo "$cleaned" | grep -v '^test ' | grep -v '^test result:' | grep -v '^running ' || true

echo -e "\n\033[0;32m> Rapport genererad: tarpaulin-report.html\033[0m"

