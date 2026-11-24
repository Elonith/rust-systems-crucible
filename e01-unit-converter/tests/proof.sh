#! /usr/bin/env bash

# This script checks wether the source code is correct using the cases.md file.
# It compares the result gotten from running the program to the one gotten using python3.
#Moreover, it checks if the Happy and Failing path both exit with the expected exit code.

cargo build --quiet

file="./tests/cases.md"
data=$(tail -n +7 $file | head -n 38 | cut -d '|' -f 3 | cut -d ' ' -f4,5,6)
py_data=$(tail -n +7 $file | head -n 38 | cut -d '|' -f 5 )

BIN="./target/debug/convert"

n=1
passed=0

# Happy path
#

echo "=== === === Happy path === === ==="

while IFS=$' \t' read -r val from_unit to_unit; do
    
    R_O=$($BIN $val $from_unit $to_unit)
    code=$?

    if ((code != 0)); then
        echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nExit_code: $code\n"
    fi

    op=$(echo "$py_data" | tail -n +$n | head -n 1)
    P_O=$(python3 -c "print(f'{$op:.2f}')")

    if [[ "$R_O" == "$P_O$to_unit" ]]; then
        #Passed.
        ((passed++)) # or passed=$((passed + 1))
    else
        echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nCause: False result\n"
    fi

    ((n++))
done <<< "$data"

# Failing path
#

echo -e "\n=== === === Failing path === === ==="
data=$(tail -n +55 $file | head -n 9 | cut -d '|' -f 3 | cut -d ' ' -f4,5,6,7)
# You can comment out the following commented lines to see the error messages.
#message=$(tail -n +55 $file | head -n 9 | cut -d '|' -f 5)

i=1
while ((i < 10)); do
    
    input=$(echo "$data" | tail -n +$i | head -n 1)
    #m=$(echo "$message" | tail -n +$i | head -n 1)
    R_O=$($BIN $input 2>&1)
    code=$?

    if ((code == 0)); then
        echo -e "$i. Computation: $BIN $input\nStatus: Failed\nExit_code: $code\n"
    fi

    #echo "=== === ==="
    #echo -e "$i. $R_O\nExit_code: $code\nMessage: $m\n"

    if (($code > 0)); then
        #Passed.
        ((passed++))
    fi
    #echo "=== === ==="

    ((i++))
done

echo -e "\n=== === === Summary === === ==="

echo -e "passed $passed / 47\nfailed $((47 - passed)) / 47"

if (( passed != 47 )); then
    exit 1
fi


