SEARCH="123456789"
REPLACE="0"

for file in $*
do
    echo "Comparing used time for file: $file"

    time tr $SEARCH $REPLACE < $file > /dev/null
    time cat $file | ../target/debug/rttr $SEARCH $REPLACE > /dev/null

    echo
    echo
    echo
done
