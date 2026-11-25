#! /usr/bin/env bash

# This script checks wether the source code is correct using the cases.md file.
# It compares the result gotten from running the program to the one gotten using python3.
#Moreover, it checks if the Happy and Failing path both exit with the expected exit code.

# Getting the absolute path of the script
script_dir="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$script_dir"

cargo build --quiet > /dev/null 2>&1

if (($? != 0)); then
    echo -e "Cargo build failed!\nExit_code: 1"
    exit 1
fi

file="cases.md"
if [[ ! -f "$file" ]]; then
    echo "$file Not found."
    cat $file > /dev/null 2>&1; echo "Exit_code: $?"
    exit 1
fi

data=$(tail -n +7 $file | head -n 38 | cut -d '|' -f 3 | cut -d ' ' -f4,5,6)
py_data=$(tail -n +7 $file | head -n 38 | cut -d '|' -f 5 )
expected_values_data=$(tail -n +7 $file | head -n 38 | cut -d '|' -f 4 )

BIN="../target/debug/convert"

n=1
passed=0
happy_cases=0

# Happy path: 38
#

echo "=== === === Happy path === === ==="

while IFS=$' \t' read -r val from_unit to_unit; do

    Rust_ouptput=$($BIN $val $from_unit $to_unit 2> /dev/null)
    code=$?

    if ((code != 0)); then
        echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nExit_code: $code\n"
        ((passed--))
    fi

    op=$(echo "$py_data" | tail -n +$n | head -n 1)
    Python3_output=$(python3 -c "print(f'{$op:.2f}')" 2> /dev/null)

    expected_value=$(echo "$expected_values_data" | tail -n +$n | head -n 1)

    # trimming trailing whitespace before comparing
    expected_value="${expected_value#"${expected_value%%[![:space:]]*}"}"
    expected_value="${expected_value%"${expected_value##*[![:space:]]}"}"

    if [[ "$Rust_ouptput" == "$Python3_output$to_unit" ]] && [[ "$Rust_ouptput" == "$expected_value" ]]; then
        #Passed.
        ((passed++)) # or passed=$((passed + 1))
    else
        if [[ -z "$expected_value" ]]; then
            echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nCause: Missing expected value\n"
        elif [[ -z "$Python3_output" ]]; then
            echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nCause: Missing computational expression\n"
        else
            echo -e "$n. Computation: $BIN $val $from_unit $to_unit\nStatus: Failed\nResult: $Rust_ouptput\nExpected result: $expected_value\nNote: Wrong expected value\n"
        fi
    fi

    ((n++))
    ((happy_cases++))
done <<< "$data"

if ((happy_cases < 38)); then
    echo -e "\nMissing rows: $((38 - happy_cases))"
fi

# Failing path: 9
#

echo -e "\n=== === === Failing path === === ==="
# Start of the failing table of data
v=$((55 - $((38 - happy_cases))))
data=$(tail -n +$v $file | head -n 9 | cut -d '|' -f 3 | cut -d ' ' -f4,5,6,7)
exit_code_data=$(tail -n +$v $file | head -n 9 | cut -d '|' -f 4)
# You can comment out the following commented lines to see the error messages.
#message=$(tail -n +55 $file | head -n 9 | cut -d '|' -f 5)
failin_cases=0
i=1
while [[ ! -z "$(echo "$data" | tail -n +$i | head -n 1)" ]]; do

    input=$(echo "$data" | tail -n +$i | head -n 1)
    exit_code=$(echo "$exit_code_data" | tail -n +$i | head -n 1)
    #m=$(echo "$message" | tail -n +$i | head -n 1)
    Rust_ouptput=$($BIN $input 2>&1)
    code=$?

    if ((code == 0)); then
        echo -e "$i. Computation: $BIN $input\nStatus: Failed\nExit_code: $code\n"

    elif ((code != exit_code)); then
        echo -e "$i. Computation: $BIN $input\nStatus: Failed\nExpected Exit_code: $exit_code\nExit_code: $code\nNote: Wrong expected Exit_code\n"
    fi

    #echo "=== === ==="
    #echo -e "$i. $Rust_ouptput\nExit_code: $code\nMessage: $m\n"

    if ((code > 0)) && ((code == exit_code)); then
            #Passed.
        ((passed++))
    fi
    #echo "=== === ==="

    ((i++))
    ((failin_cases++))
    
done

if ((failin_cases < 9)); then
    echo -e "\nMissing rows: $((9 - failin_cases))"
fi

tot=$((happy_cases + failin_cases))

echo -e "\n=== === === Summary === === ==="

echo -e "passed $passed / 47\nfailed $((tot - passed)) / 47"

if ((tot != 47)) || ((passed != tot)); then
    echo -e "\nTotal tests carried out: $tot instead of 47"
    exit 1
fi


